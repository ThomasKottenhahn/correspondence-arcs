use std::collections::HashMap;

use crate::data::game_state::{GameState,Trophy, ReserveType, Color, PlayerArea};
use crate::data::court_cards::{CourtCard, Guild, VoxPayload};

pub(crate) fn secure(game_state: &GameState, target_card: u8, vox_payload: Option<VoxPayload>) -> GameState {
    let current_player = game_state.current_player.clone();
    let card = game_state.court[target_card as usize].clone();
    let new_game_state = game_state.clone();
    
    if card.controlled_by() == Some(current_player.clone()) {
        match card {
            CourtCard::VoxCard { vox, agents } => {
                let tropies: Vec<Trophy> = agents.iter()
                    .filter(|a| a.color != current_player)
                    .map(|a| Trophy{trophy_type: ReserveType::Agents, count: a.count, player: a.color.clone()})
                    .collect();
                let players_agents = agents.iter().find(|a| a.color == current_player).unwrap().count;
                let current_player_area = new_game_state.get_player_area(&current_player);
                let combined_trophies = current_player_area.add_trophies(tropies);

                let new_players: HashMap<Color, PlayerArea> = new_game_state.players.iter().map(|(c,p)| if *c==current_player 
                                    {
                                        (c.clone(), PlayerArea{ 
                                            tropies: combined_trophies.clone(),
                                            ..p.clone()
                                            })
                                    }
                                    else {(c.clone(),p.clone())}
                                ).collect();

                let mut new_game_state = GameState { players: new_players, ..new_game_state.clone() };
                new_game_state.update_players_reserve(&current_player, &ReserveType::Agents, players_agents as i8);

                return (vox.on_secure)(&new_game_state, vox_payload.expect("VoxPayload required"));
            },
            CourtCard::GuildCard { guild, agents } => {
                let tropies: Vec<Trophy> = agents.iter()
                    .filter(|a| a.color != current_player)
                    .map(|a| Trophy{trophy_type: ReserveType::Agents, count: a.count, player: a.color.clone()})
                    .collect();
                let players_agents = agents.iter().find(|a| a.color == current_player).unwrap().count;
                let current_player_area = new_game_state.get_player_area(&current_player);
                let combined_trophies = current_player_area.add_trophies(tropies);
                let new_guild_cards: Vec<Guild> = current_player_area.guild_cards.iter().cloned().chain(vec![guild.clone()]).collect();
                let new_players: HashMap<Color, PlayerArea> = new_game_state.players.iter().map(|(c,p)| if *c==current_player 
                                    { (c.clone(), PlayerArea{ 
                                        tropies: combined_trophies.clone(),
                                        guild_cards: new_guild_cards.clone(),
                                        ..p.clone()
                                    }
                                )}
                                    else {(c.clone(),p.clone())}
                                ).collect();
                let mut new_game_state = GameState { players: new_players, ..new_game_state.clone() };
                new_game_state.update_players_reserve(&current_player, &ReserveType::Agents, players_agents as i8);

                return new_game_state;
            },
        }
    }
    else { panic!("Can only secure controlled Card") };
}
