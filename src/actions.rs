use crate::board::update_control;
use crate::data::GameState;
use crate::data::Color;
use crate::data::Action;
use crate::data::Action::Build;
use crate::data::BuildType;
use crate::data::BuildingType;
use crate::data::Ships;
use crate::data::System;
use crate::data::BuildingSlot;

fn place_building(building_slots: &Vec<BuildingSlot>, building: BuildingSlot) -> Vec<BuildingSlot>{ 
    for i in 0..building_slots.len(){
        if building_slots[i] == BuildingSlot::Empty{
            let mut building_slots = building_slots.clone();
            building_slots[i] = building;
            return building_slots;
        }
    }
    return building_slots.clone();
}

fn place_ship(ships: &Vec<Ships>, player: Color, fresh: bool) -> Vec<Ships>{
    let mut new_ships: Vec<Ships> = vec![];
    for ships_struct in ships{
        if ships_struct.player == player {
            let mut ships_struct = ships_struct.clone();
            if fresh {ships_struct.fresh = ships_struct.fresh + 1}
            else {ships_struct.damaged = ships_struct.damaged + 1}
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
        System::Unused => return game_state.clone(),
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
                    ships: place_ship(&ships, current_player, build_fresh), 
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

fn execute_action(game_state: &GameState, action: Action) -> GameState {
    return match action {
        Build {target_system: s, build_type: b} => build(game_state, s, b),
        _ => game_state.clone()
    };
}