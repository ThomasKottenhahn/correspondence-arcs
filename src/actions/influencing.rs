use crate::data::game_state::{GameState, Agents, ReserveType};
use crate::data::court_cards::CourtCard;

pub(crate) fn influence(game_state: &GameState, target_card: u8) -> GameState {
    let current_player = game_state.current_player.clone();
    let mut new_game_state = game_state.clone();

    new_game_state.court[target_card as usize] = match new_game_state.court[target_card as usize].clone() {
        CourtCard::VoxCard { vox, agents } => CourtCard::VoxCard {vox: vox, agents: agents.iter().map(|a| {
            if a.color == current_player {Agents{color: current_player.clone(), count: a.count + 1}}
            else {a.clone()}
        }).collect()},
        CourtCard::GuildCard { guild, agents } => CourtCard::GuildCard {guild: guild, agents: agents.iter().map(|a| {
            if a.color == current_player {Agents{color: current_player.clone(), count: a.count + 1}}
            else {a.clone()}
        }).collect()},
    };

    new_game_state.update_players_reserve(&current_player, &ReserveType::Agents, -1);

    return new_game_state;
}
