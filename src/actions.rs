use rand::Rng;

use crate::data::ActionType;
use crate::data::Agents;
use crate::data::Dice;
use crate::data::GameState;
use crate::data::Color;
use crate::data::Action;
use crate::data::Trophy;
use crate::data::TrophyType;
use crate::data::TurnState;
use crate::data::BuildType;
use crate::data::BuildingType;
use crate::data::Ships;
use crate::data::System;
use crate::data::BuildingSlot;
use crate::data::CourtCard;
use crate::data::VoxPayload;

pub fn place_building(building_slots: &Vec<BuildingSlot>, building: BuildingSlot) -> Vec<BuildingSlot> {
    if building_slots.len() == 0{
        panic!("No building slots available");
    }

    if building_slots.iter().all(|x| matches!(x, BuildingSlot::Occupied { .. })) {
        panic!("All building slots are occupied");
    }

    for i in 0..building_slots.len(){
        if building_slots[i] == BuildingSlot::Empty{
            let mut building_slots = building_slots.clone();
            building_slots[i] = building;
            return building_slots;
        }
    }
    return building_slots.clone();
}

pub fn place_ships(ships: &Vec<Ships>, player: Color, fresh: u8, damaged: u8) -> Vec<Ships> {
    let mut new_ships: Vec<Ships> = vec![];
    for ships_struct in ships{
        if ships_struct.player == player {
            let mut ships_struct = ships_struct.clone();
            ships_struct.fresh = ships_struct.fresh + fresh;
            ships_struct.damaged = ships_struct.damaged + damaged;
            new_ships.push(ships_struct);
        } else {
            new_ships.push(ships_struct.clone());
        }
    }
    return new_ships;
}

fn remove_ships(ships: &Vec<Ships>, player: Color, fresh: u8, damaged: u8) -> Vec<Ships> {
    let mut new_ships: Vec<Ships> = vec![];
    for ships_struct in ships{
        if ships_struct.player == player {
            let mut ships_struct = ships_struct.clone();
            ships_struct.fresh = ships_struct.fresh.checked_sub(fresh).expect("Tried to move more fresh ships than available");
            ships_struct.damaged = ships_struct.damaged.checked_sub(damaged).expect("Tried to move more damaged ships than available");
            new_ships.push(ships_struct);
        } else {
            new_ships.push(ships_struct.clone());
        }
    }
    return new_ships;
}

fn build(game_state: &GameState, target_system: u8, build_type: BuildType) -> GameState {
    let mut game_state = game_state.clone();

    let current_player = game_state.current_player.clone();
    let system: System = game_state.systems[target_system as usize].clone();
    
    if !system.has_presence(&current_player) {panic!("Cannot build in a System without presence")}
    
    match system{
        System::Unused => panic!("System is unused"),
        System::Used {
            system_id,
            system_type,
            building_slots,
            ships,
            controlled_by,
            connects_to
        } => {
            let build_fresh = match controlled_by.clone() {
                None => true,
                Some(c) => c==current_player
            };    
        
            match build_type {
                BuildType::Ship => game_state.systems[target_system as usize] = System::Used {
                    system_id: system_id, 
                    system_type: system_type.clone(), 
                    building_slots: building_slots.clone(), 
                    ships: place_ships(&ships, current_player, if build_fresh {1} else {0}, if !build_fresh {0} else {0}), 
                    controlled_by: controlled_by.clone(), 
                    connects_to: connects_to.clone() }.update_control(),
                BuildType::City => {
                    let building = BuildingSlot::Occupied {fresh: build_fresh, player: current_player, building_type: BuildingType::City, used: false};
                    game_state.systems[target_system as usize] = System::Used {
                    system_id: system_id, 
                    system_type: system_type.clone(), 
                    building_slots: place_building(&building_slots, building), 
                    ships: ships.clone(), 
                    controlled_by: controlled_by.clone(), 
                    connects_to: connects_to.clone() }
                },   
                BuildType::Starport => {
                    let building = BuildingSlot::Occupied {fresh: build_fresh, player: current_player, building_type: BuildingType::Starport, used: false};
                    game_state.systems[target_system as usize] = System::Used {
                    system_id: system_id, 
                    system_type: system_type.clone(), 
                    building_slots: place_building(&building_slots, building), 
                    ships: ships.clone(), 
                    controlled_by: controlled_by.clone(), 
                    connects_to: connects_to.clone() }
                }
            }
            return game_state;
        }
    }
    
}

pub fn move_ships(game_state: &GameState, origin_system_id: u8, destination_system_id: u8, fresh: u8, damaged: u8) -> GameState {
    let mut game_state = game_state.clone();

    let origin_system = game_state.systems[origin_system_id as usize].clone();
    let destination_system = game_state.systems[destination_system_id as usize].clone();

    match origin_system{
        System::Unused => panic!("Origin system is unused"),
        System::Used {
            system_id,
            system_type,
            building_slots,
            ships,
            controlled_by,
            connects_to
        } => {
            let updated_ships = remove_ships(&ships, game_state.current_player.clone(), fresh, damaged);
            game_state.systems[origin_system_id as usize] = System::Used {
                system_id,
                system_type: system_type.clone(),
                building_slots: building_slots.clone(),
                ships: updated_ships,
                controlled_by: controlled_by.clone(),
                connects_to: connects_to.clone(),
            }.update_control();
        }
    }

    match destination_system{
        System::Unused => panic!("Destination system is unused"),
        System::Used {
            system_id,
            system_type,
            building_slots,
            ships,
            controlled_by,
            connects_to
        } => {
            let updated_ships = place_ships(&ships, game_state.current_player.clone(), fresh, damaged);
            game_state.systems[destination_system_id as usize] = System::Used {
                system_id: system_id, 
                system_type: system_type.clone(), 
                building_slots: building_slots.clone(), 
                ships: updated_ships, 
                controlled_by: controlled_by.clone(), 
                connects_to: connects_to.clone() }.update_control()
        }
    }

    return game_state;
}

fn repair(game_state: &GameState, target_system: u8, build_type: BuildType) -> GameState {
    let mut game_state = game_state.clone();
    let system = game_state.systems[target_system as usize].clone();
    let current_player = game_state.current_player.clone();
    match system {
        System::Unused => panic!("Cannot repair in unused System"),
        System::Used {
            system_id,
            system_type,
            mut building_slots,
            ships,
            controlled_by,
            connects_to
        } => match build_type {
            BuildType::Ship => {
                let ships = remove_ships(&ships, current_player.clone(), 0, 1);
                let ships = place_ships(&ships, current_player.clone(), 1, 0);
                game_state.systems[target_system as usize] = System::Used {
                    system_id: system_id,
                    system_type: system_type,
                    building_slots: building_slots,
                    ships: ships,
                    controlled_by: controlled_by,
                    connects_to: connects_to
                }.update_control();
            },
            BuildType::City => {
                match (building_slots[0].clone(), building_slots[1].clone()) {
                    (BuildingSlot::Occupied { fresh: false, player: current_player, building_type: BuildingType::City, used }, _) => building_slots[0] = BuildingSlot::Occupied { fresh: true, player: current_player, building_type: BuildingType::City, used },
                    (_, BuildingSlot::Occupied { fresh: false, player: current_player, building_type: BuildingType::City, used }) => building_slots[1] = BuildingSlot::Occupied { fresh: true, player: current_player, building_type: BuildingType::City, used },
                    _ => panic!("Cannot repair, because no valid repair target exists")
                }
            },
            BuildType::Starport => {
                match (building_slots[0].clone(), building_slots[1].clone()) {
                    (BuildingSlot::Occupied { fresh: false, player: current_player, building_type: BuildingType::Starport, used }, _) => building_slots[0] = BuildingSlot::Occupied { fresh: true, player: current_player, building_type: BuildingType::Starport, used },
                    (_, BuildingSlot::Occupied { fresh: false, player: current_player, building_type: BuildingType::Starport, used }) => building_slots[1] = BuildingSlot::Occupied { fresh: true, player: current_player, building_type: BuildingType::Starport, used },
                    _ => panic!("Cannot repair, because no valid repair target exists")
                }
            }
        }
    }
    return game_state;
}

fn influence(game_state: &GameState, target_card: u8) -> GameState {
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
    return new_game_state;
}

fn battle(game_state: &GameState, target_system: u8, target_player: Color, dice: Vec<Dice>) -> GameState {
    let mut rng = rand::thread_rng();
    let (self_hits, intecept, hits, building_hits, keys) = dice.iter().map(|d| {
        let faces: Vec<(u8, u8, u8, u8, u8)> = match d {
            Dice::Skirmish => vec![(0,0,1,0,0), (0,0,1,0,0), (0,0,1,0,0), (0,0,0,0,0), (0,0,0,0,0), (0,0,0,0,0)],
            Dice::Assault => vec![(0,0,2,0,0), (1,0,2,0,0), (0,1,1,0,0), (1,0,1,0,0), (1,0,1,0,0), (0,0,0,0,0)],
            Dice::Raid => vec![(0,1,0,0,2), (1,0,0,0,1), (0,0,0,1,1), (1,0,0,1,0), (1,0,0,1,0), (0,1,0,0,0)],
        };
        let roll = rng.gen_range(0..faces.len());
        faces[roll]
    }).fold((0, 0, 0, 0, 0), |acc, x| (
        acc.0 + x.0,
        acc.1 + x.1,
        acc.2 + x.2,
        acc.3 + x.3,
        acc.4 + x.4
    ));

    let self_hits = if intecept >= 1 {self_hits + game_state.systems[target_system as usize].get_fresh_ships(&target_player)} else {self_hits};
    let mut new_game_state = game_state.clone();
    new_game_state.next_turn_state = Some(new_game_state.turn_state);
    new_game_state.turn_state = TurnState::AllocateDiceResults { target_system: target_system, target_player: target_player, self_hits: self_hits, hits: hits, building_hits: building_hits, keys: keys };
    return new_game_state;
}

fn secure(game_state: &GameState, target_card: u8, vox_payload: Option<VoxPayload>) -> GameState {
    let current_player = game_state.current_player.clone();
    let card = game_state.court[target_card as usize].clone();
    let mut new_game_state = game_state.clone();
    
    if card.controlled_by() == Some(current_player.clone()) {
        match card {
            CourtCard::VoxCard { vox, agents } => {
                let tropies: Vec<Trophy> = agents.iter()
                    .filter(|a| a.color != current_player)
                    .map(|a| Trophy{trophy_type: TrophyType::Agent, count: a.count, player: a.color.clone()})
                    .collect();
                let new_player_area = new_game_state.get_player_area(&current_player).add_trophies(tropies);
                let player_area_index = new_game_state.get_player_area_ind(&current_player) as usize;
                new_game_state.players[player_area_index] = new_player_area;

                return (vox.on_secure)(&new_game_state, vox_payload.expect("VoxPayload required"));
            },
            CourtCard::GuildCard { guild, agents } => {
                let tropies: Vec<Trophy> = agents.iter()
                    .filter(|a| a.color != current_player)
                    .map(|a| Trophy{trophy_type: TrophyType::Agent, count: a.count, player: a.color.clone()})
                    .collect();
                let mut new_player_area = new_game_state.get_player_area(&current_player).add_trophies(tropies);
                let player_area_index = new_game_state.get_player_area_ind(&current_player) as usize;
                new_player_area.guild_cards.push(guild);
                new_game_state.players[player_area_index] = new_player_area;
                return new_game_state;
            },
        }
    }
    else {panic!("Can only secure controlled Card")};
}

pub fn execute_actions(game_state: &GameState, actions: Vec<Action>) -> GameState {
    return actions
        .iter()
        .fold(game_state.clone(), |g, action| execute_action(&g, action.clone()));
}

pub fn execute_action(game_state: &GameState, action: Action) -> GameState {
    match &game_state.turn_state {
        TurnState::TrickTaking => {
            match action {
                Action::PlayLeadCard { card, declare } => {
                    let mut new_game_state = game_state.clone();
                    new_game_state.lead_card = Some(card.clone());
                    match declare {
                        Some(_) => todo!(),
                        None => {},
                    }
                    new_game_state.turn_state = TurnState::Prelude { action_type: card.action_type, pips_left: card.pips };
                    return new_game_state;
                },
                Action::Surpass { card, seize } => todo!(),
                Action::Copy { card, seize } => todo!(),
                Action::Pivot { card, seize } => todo!(),
                _ => panic!("Can only Execute TrickTaking Actions")
            }
        },
        TurnState::Prelude { action_type, pips_left } => {
            match action {
                Action::EndPrelude => {
                    let mut new_game_state = game_state.clone();
                    new_game_state.turn_state = TurnState::Actions { action_type: action_type.clone(), pips_left: pips_left.clone() };
                    return new_game_state;
                },
                _ => todo!()
            }
        },
        TurnState::Actions { action_type, pips_left } => {
            if *pips_left == 0 {panic!("No Action pips left")}
            match action_type {
                ActionType::Administration => match action {
                Action::Repair { target_system, build_type } => repair(game_state, target_system, build_type),
                Action::Tax { target_system, resource } => todo!(),
                Action::Influence { card_id } => influence(game_state, card_id),
                _ => panic!("Cannot execute Action with Administration Action Card")
            },
                ActionType::Agression =>  match action {
                Action::Move { origin_id, destination_id, fresh_ships, damaged_ships } => move_ships(game_state, origin_id, destination_id, fresh_ships, damaged_ships),
                Action::Secure { card_id , vox_payload} => secure(game_state, card_id, vox_payload),
                Action::Battle { target_system, target_player, dice } => battle(game_state, target_system, target_player, dice),
                _ => panic!("Cannot execute Action with Aggresion Action Card")
            },
                ActionType::Construction => match action {
                Action::Build {target_system, build_type} => build(game_state, target_system, build_type),
                Action::Repair { target_system, build_type } => repair(game_state, target_system, build_type),
                _ => panic!("Cannot execute Action with Construction Action Card")
            },
                ActionType::Mobilization => match action {
                Action::Move { origin_id, destination_id, fresh_ships, damaged_ships } => move_ships(game_state, origin_id, destination_id, fresh_ships, damaged_ships),
                Action::Influence { card_id } => influence(game_state, card_id),
                _ => panic!("Cannot execute Action with Mobilization Action Card")
            },
            }
        },
        TurnState::AllocateResource { resource } => todo!(),
        TurnState::AllocateDiceResults { target_system, target_player, self_hits, hits, building_hits, keys } => todo!(),
    }
}