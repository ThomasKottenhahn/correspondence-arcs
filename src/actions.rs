use crate::board::update_control;
use crate::data::GameState;
use crate::data::Color;
use crate::data::Action;
use crate::data::BuildType;
use crate::data::BuildingType;
use crate::data::Ships;
use crate::data::System;
use crate::data::BuildingSlot;

fn place_building(building_slots: &Vec<BuildingSlot>, building: BuildingSlot) -> Vec<BuildingSlot>{
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

fn place_ships(ships: &Vec<Ships>, player: Color, fresh: u8, damaged: u8) -> Vec<Ships>{
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

fn remove_ships(ships: &Vec<Ships>, player: Color, fresh: u8, damaged: u8) -> Vec<Ships>{
    let mut new_ships: Vec<Ships> = vec![];
    for ships_struct in ships{
        if ships_struct.player == player {
            let mut ships_struct = ships_struct.clone();
            ships_struct.fresh = ships_struct.fresh - fresh;
            ships_struct.damaged = ships_struct.damaged - damaged;
            new_ships.push(ships_struct);
        } else {
            new_ships.push(ships_struct.clone());
        }
    }
    return new_ships;
}

pub fn build(game_state: &GameState, target_system: u8, build_type: BuildType) -> GameState{
    let mut game_state = game_state.clone();

    let current_player = game_state.current_player.clone();
    let system: System = game_state.systems[target_system as usize].clone();

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
                BuildType::Ship => game_state.systems[target_system as usize] = update_control(&System::Used {
                    system_id: system_id, 
                    system_type: system_type.clone(), 
                    building_slots: building_slots.clone(), 
                    ships: place_ships(&ships, current_player, if build_fresh {1} else {0}, if !build_fresh {0} else {0}), 
                    controlled_by: controlled_by.clone(), 
                    connects_to: connects_to.clone() }
                ),
                BuildType::City => {
                    let building = BuildingSlot::Occupied {fresh: build_fresh, player: current_player, building_type: BuildingType::City};
                    game_state.systems[target_system as usize] = System::Used {
                    system_id: system_id, 
                    system_type: system_type.clone(), 
                    building_slots: place_building(&building_slots, building), 
                    ships: ships.clone(), 
                    controlled_by: controlled_by.clone(), 
                    connects_to: connects_to.clone() }
                },   
                BuildType::Starport => {
                    let building = BuildingSlot::Occupied {fresh: build_fresh, player: current_player, building_type: BuildingType::Starport};
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

pub fn move_ships(game_state: &GameState, origin_system_id: u8, destination_system_id: u8, fresh: u8, damaged: u8) -> GameState{
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
            game_state.systems[origin_system_id as usize] = update_control(&System::Used {
                system_id,
                system_type: system_type.clone(),
                building_slots: building_slots.clone(),
                ships: updated_ships,
                controlled_by: controlled_by.clone(),
                connects_to: connects_to.clone(),
            });
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
            game_state.systems[destination_system_id as usize] = update_control(&System::Used {
                system_id: system_id, 
                system_type: system_type.clone(), 
                building_slots: building_slots.clone(), 
                ships: updated_ships, 
                controlled_by: controlled_by.clone(), 
                connects_to: connects_to.clone() })
        }
    }

    return game_state;
}

pub fn execute_action(game_state: &GameState, action: Action) -> GameState {
    return match action {
        Action::Build {target_system, build_type} => build(game_state, target_system, build_type),
        Action::Move { origin_id, destination_id, fresh_ships, damaged_ships } => move_ships(game_state, origin_id, destination_id, fresh_ships, damaged_ships),
        _ => game_state.clone()
    };
}