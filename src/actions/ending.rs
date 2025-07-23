use std::collections::HashMap;

use itertools::Itertools;
use crate::data::game_state::{Agents, AmbitionTypes, Color, GameState, PlayerArea, ReserveType, ResourceType, Trophy, TurnState};

pub(crate) fn score_ambition(game_state: &GameState, ambition: AmbitionTypes) -> GameState {
    let evaluation = match ambition {
        AmbitionTypes::Tycoon => |a: PlayerArea| a.get_resource_count(ResourceType::Fuel) + a.get_resource_count(ResourceType::Material),
        AmbitionTypes::Tyrant => |a: PlayerArea| a.get_captives(),
        AmbitionTypes::Warlord => |a: PlayerArea| a.get_trophies(),
        AmbitionTypes::Keeper => |a: PlayerArea| a.get_resource_count(ResourceType::Relics),
        AmbitionTypes::Empath => |a: PlayerArea| a.get_resource_count(ResourceType::Psionics),
    };
    let ranking = game_state.players.iter()
        .map(|(c,a)| (Some(c.clone()), evaluation(a.clone())))
        .chain(
            vec![(None, game_state.ambitions.get(&ambition).unwrap().discarded_resources.len() as u8)]
        )
        .sorted_by(|a, b| b.1.cmp(&a.1))
        .collect::<HashMap<Option<Color>, u8>>();

    let points: HashMap<Option<Color>, usize> = ranking
        .iter()
        .map(|(c,s)|{
            match c {
                Some(c) => {
                    let higher_count = ranking.iter().filter(|(_c2, s2)| *s2 > s).count();
                    if higher_count == 0 && ranking.iter().all(|(_c2,s2)| s2 < s) {
                        // This player is the sole highest
                        let points = game_state.ambitions
                            .get(&ambition)
                            .unwrap()
                            .markers
                            .iter()
                            .map(|m| if m.flipped {m.first_place_flipped as usize} else {m.first_place as usize}).sum::<usize>()
                            + match game_state.players.get(&c).unwrap().reserve.get(&ReserveType::Cities).unwrap() {
                                0 => 5,
                                1 => 2,
                                _ => 0
                            };
                        (Some((*c).clone()), points)
                    } else if higher_count == 1 || higher_count == 0 {
                        // Only one player is higher, so this is second place or tied first
                        let points = game_state.ambitions
                            .get(&ambition)
                            .unwrap()
                            .markers
                            .iter()
                            .map(|m| if m.flipped {m.second_place_flipped as usize} else {m.second_place as usize}).sum();
                        (Some((*c).clone()), points)
                    } else {
                        (Some((*c).clone()), 0)
                    }
                },
                None => (None, 0)
            }
        })
        .filter(|(c,_s)| *c != None)
        .collect();

    let new_players: HashMap<Color, PlayerArea> = game_state.players
        .iter()
        .map(|(c,a)| (c.clone(), PlayerArea { power: a.power + *points.get(&Some(c.clone())).unwrap() as u8, ..a.clone()}))
        .collect();
    
    if ambition == AmbitionTypes::Warlord {
        let trophies: Vec<Trophy> = game_state.players
            .iter()
            .map(|(_,a)| a.tropies.clone())
            .flatten()
            .collect();
        
        let mut new_game_state = game_state.clone();

        for t in trophies {
            new_game_state.update_players_reserve(&t.player, &t.trophy_type, t.count as i8);
        }

        return new_game_state;
        
    }

    if ambition == AmbitionTypes::Tyrant {
        let captives: Vec<Agents> = game_state.players
            .iter()
            .map(|(_,a)| a.captives.clone())
            .flatten()
            .collect();
        let mut new_game_state = game_state.clone();

        for a in captives {
            new_game_state.update_players_reserve(&a.color, &ReserveType::Agents, a.count as i8);
        }

        return new_game_state;
    }

    GameState {players: new_players, .. game_state.clone()}
}

pub(crate) fn end_chapter(game_state: &GameState) -> GameState {
    let new_game_state = game_state.ambitions.iter()
        .filter(|(_, a)| a.markers.len() != 0)
        .map(|(t, a)| (t.clone(), a.clone()))
        .fold(game_state.clone(), |gs, (ambition_type, _)| {
            score_ambition(&gs, ambition_type.clone())
        });

    //Todo reshuffle cards

    println!("End Chapter");
    return new_game_state;
}

pub(crate) fn end_round(game_state: &GameState) -> GameState {
    println!("Ending Round");
    //determine new Initiative, discard Cards
    let mut new_game_state = game_state.clone();

    let lead = game_state.lead_card.as_ref().unwrap();
    let follow_cards = game_state.follow_cards.clone();

    new_game_state.initiative =  match new_game_state.seized.clone() {
        Some(c) => c,
        None => {
            game_state.follow_cards.iter().chain(vec![lead]).filter(|(c, f, _)| c.action_type == lead.0.action_type && *f).max_by_key(|(c, _, _)| c.number).unwrap().2.clone()
        }
    };

    new_game_state.current_player = new_game_state.initiative.clone();

    new_game_state.action_discard.push(lead.0.clone());
    new_game_state.action_discard = new_game_state.action_discard.iter().cloned().chain(follow_cards.iter().map(|(c, _, _)| c.clone())).collect();
    new_game_state.follow_cards = vec![];
    new_game_state.turn_state = TurnState::TrickTaking;
    new_game_state.players_in_round = new_game_state.players.iter().filter(|(_, a)| a.action_cards.len() != 0).count() as u8;

    if new_game_state.players.iter().filter(|(_, area)| area.action_cards.len() != 0).count() == 0 {end_chapter(&new_game_state)} else {new_game_state}
}

pub(crate) fn end_turn(game_state: &GameState) -> GameState {
    let player_order: Vec<Color> = vec![Color::Red, Color::Blue, Color::White, Color::Yellow].iter().take(game_state.players.len()).cloned().collect();
    let mut new_game_state = game_state.clone();
    new_game_state.turn_state = TurnState::TrickTaking;
    new_game_state.current_player = player_order[(player_order.iter().position(|c| *c == new_game_state.current_player).unwrap() + 1)%player_order.len()].clone();
    new_game_state.systems = new_game_state.systems.iter().map(|s| s.refresh_buildings()).collect();

    // last player in Turn Order
    if (1 + game_state.follow_cards.len()) as u8 == game_state.players_in_round {
        return end_round(&new_game_state);
    } else {
        new_game_state
    }
}
