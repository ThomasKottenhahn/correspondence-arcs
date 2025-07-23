use crate::data::game_state::{ActionCard, Ambition, AmbitionTypes, GameState, TurnState};

pub(crate) fn play_lead_card(game_state: &GameState, card: ActionCard, declare: Option<AmbitionTypes>) -> GameState {
    let mut new_game_state = game_state.clone();
    let mut player_area = new_game_state.get_player_area(&new_game_state.current_player);
    player_area.remove_action_card(card.clone());
    new_game_state.players.insert(new_game_state.current_player.clone(), player_area);
    new_game_state.lead_card = Some((card.clone(), true, game_state.current_player.clone()));
    new_game_state.turn_state = TurnState::Prelude { action_type: card.action_type.clone(), pips_left: card.pips };
    match declare {
        Some(ambition) => {
            match &card.declared_ambition {
                Some(possible_declare) => if possible_declare == &ambition 
                        {declare_ambition(game_state, ambition)}
                    else
                        {panic!("AmbitionType on card {:?} does not match Ambition {:?}", &card, ambition)},
                None => panic!("Cannot declare Ambition with {:?}", card),
            }
        },
        None => new_game_state 
    }
}

pub(crate) fn surpass(game_state: &GameState, card: ActionCard, seize: Option<ActionCard>) -> GameState {
    let (lead_card, _, _) = game_state.lead_card.as_ref().expect("Cannot Surpass, if no lead card exists");
    if card.action_type != lead_card.action_type {panic!("Cannot surpass with other card type")}
    if card.number < lead_card.number {panic!("Cannot surpass with a lower card")};
    let mut new_game_state = game_state.clone();
    let mut player_area = new_game_state.get_player_area(&new_game_state.current_player);
    player_area.remove_action_card(card.clone());
    new_game_state.players.insert(new_game_state.current_player.clone(), player_area);
    new_game_state.follow_cards.push((card.clone(), true, game_state.current_player.clone()));
    match seize {
        Some(_) => todo!(),
        None => {},
    }
    new_game_state.turn_state = TurnState::Prelude { action_type: card.action_type, pips_left: card.pips };
    new_game_state
}

pub(crate) fn copy(game_state: &GameState, card: ActionCard, seize: Option<ActionCard>) -> GameState {
    let (lead_card, _, _) = game_state.lead_card.as_ref().unwrap();
    let mut new_game_state = game_state.clone();
    let mut player_area = new_game_state.get_player_area(&new_game_state.current_player);
    player_area.remove_action_card(card.clone());
    new_game_state.players.insert(new_game_state.current_player.clone(), player_area);
    new_game_state.follow_cards.push((card.clone(), false, game_state.current_player.clone()));
    match seize {
        Some(_) => new_game_state.seized = Some(new_game_state.current_player.clone()),
        None => {},
    }
    new_game_state.turn_state = TurnState::Prelude { action_type: lead_card.action_type.clone(), pips_left: 1 };
    new_game_state
}

pub(crate) fn pivot(game_state: &GameState, card: ActionCard, seize: Option<ActionCard>) -> GameState {
    let (lead_card, _, _) = game_state.lead_card.as_ref().unwrap();
    if card.action_type == lead_card.action_type {panic!("Cannot Pivot with same card type")};
    let mut new_game_state = game_state.clone();
    let mut player_area = new_game_state.get_player_area(&new_game_state.current_player);
    player_area.remove_action_card(card.clone());
    new_game_state.players.insert(new_game_state.current_player.clone(), player_area);
    new_game_state.follow_cards.push((card.clone(), true, game_state.current_player.clone()));
    match seize {
        Some(_) => new_game_state.seized = Some(new_game_state.current_player.clone()),
        None => {},
    }
    new_game_state.turn_state = TurnState::Prelude { action_type: card.action_type, pips_left: 1 };
    new_game_state
}

fn declare_ambition(game_state: &GameState, ambition: AmbitionTypes) -> GameState {
    let highest_ambition = game_state.ambition_markers.iter().max_by_key(|am| if am.flipped {am.first_place_flipped} else {am.first_place});
    let highest_ambition = match highest_ambition {
        Some(a) => a,
        None => panic!("Cannot declare, because no AmbitionMarker is available")
    };
    let mut ambitions = game_state.ambitions.clone();
    let ambition_box = ambitions.get(&ambition).unwrap();
    ambitions.insert(ambition, Ambition{markers: ambition_box.markers.iter().chain(vec![highest_ambition]).cloned().collect(), .. ambition_box.clone()});
    GameState{
        ambition_markers: game_state.ambition_markers.iter().filter(|am| am != &highest_ambition).cloned().collect(),
        ambitions: ambitions,
        .. game_state.clone()
    }
}