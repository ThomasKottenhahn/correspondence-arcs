use std::clone;

use super::game_state::{GameState, Color, Agents, ResourceType, AmbitionTypes, PreludeActionPayload};
use super::system::System;
use crate::actions::place_ships;
use crate::data::game_state::ReserveType;
use crate::board::get_cluster;

use itertools::Itertools;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CourtCard {
    VoxCard {vox: Vox, agents: Vec<Agents> },
    GuildCard {guild: Guild, agents: Vec<Agents>}
}

impl CourtCard {
    pub fn controlled_by(&self) -> Option<Color> {
        match self {
            CourtCard::VoxCard { agents, .. } | CourtCard::GuildCard { agents, .. } => {
                agents.iter().fold(None, |acc: Option<(Color, u8)>, agent| {
                    match acc {
                        None => if agent.count > 0 {Some((agent.color.clone(), agent.count))} else {None},
                        Some((ref max_color, max_count)) => {
                            if agent.count > max_count {
                                Some((agent.color.clone(), agent.count))
                            } else if agent.count == max_count && agent.color != *max_color {
                                None
                            } else {
                                Some((max_color.clone(), max_count))
                            }
                        }
                    }
                }).map(|(color, _)| color)
            }
        }
    }
}

pub fn create_court_deck(players: Vec<Color>) -> Vec<CourtCard> {
    let agents: Vec<Agents> = players.iter().map(|color| Agents { color: color.clone(), count: 0 }).collect();
    vec![
        CourtCard::VoxCard {
            vox: Vox {
                title: "Mass Uprising".to_string(),
                description: "When Secured: Choose a cluster on the map. You place 1 ship in each system of that cluster. Discard this card.".to_string(),
                on_secure: mass_uprising
            },
            agents: agents.clone()
        }
    ]
}

fn mass_uprising(game_state: &GameState, vox_payload: VoxPayload) -> GameState {
    match vox_payload {
        VoxPayload::MassUprising { target_systems } => {
            let reserve_ships = game_state.players.get(&game_state.current_player).unwrap().reserve.get(&ReserveType::Ships).unwrap();


            if target_systems.len() != 4 && *reserve_ships >= 4 {
                panic!("Invalid number of target systems for mass uprising: {:?}", target_systems);
            } 

            if target_systems.len() > (*reserve_ships).into() {
                panic!("Cannot place {:?} Ships using Mass Uprising, when only {:?} are in reserve", target_systems.len(), reserve_ships)
            }

            //check if all target systems are in same cluster
            if !target_systems.iter().map(|s| get_cluster(*s)).tuple_windows().all(|(a, b)| a == b) {
                panic!("Systems are not all in the same cluster")
            }


            let new_systems = game_state.systems.iter().map(|s| {
                match s {
                    System::Used { 
                        system_id,
                        system_type,
                        building_slots,
                        ships,
                        controlled_by,
                        connects_to } if target_systems.contains(system_id) => System::Used { 
                            system_id: *system_id,
                            system_type: system_type.clone(),
                            building_slots: building_slots.clone(),
                            ships: place_ships(ships, &game_state.current_player, 1, 0),
                            controlled_by: controlled_by.clone(),
                            connects_to: connects_to.clone() 
                        }.update_control(),
                    _ => s.clone()
                }
            });
            
            GameState {
                systems: new_systems.collect(),
                ..game_state.clone()
            }.update_players_reserve(&game_state.current_player, &ReserveType::Ships, -(target_systems.len() as i8))
        }
        _ => panic!("Cannot call mass_uprising with {:?}", vox_payload)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Vox {
    pub title: String,
    pub description: String,
    pub on_secure: fn(&GameState, VoxPayload) -> GameState
}
    
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Guild {
    title: String,
    description: String,
    resource: ResourceType,
    keys: u8,
    prelude_action: fn(&GameState, PreludeActionPayload) -> GameState
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VoxPayload {
    MassUprising {target_systems: Vec<u8>},
    PopulistDemands {ambition: Option<AmbitionTypes>},
    OutrageSpreads {outrage_type: Option<ResourceType>},
    SongOfFreedom {target_system: Option<u8>, target_player: Option<Color>},
    GuildStruggle {target_player: Option<Color>, target_card: Option<u8>},
    CallToAction
}