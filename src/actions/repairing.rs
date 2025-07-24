use crate::data::game_state::{GameState, BuildType};
use crate::data::system::{BuildingSlot, System, BuildingType};

use crate::board::{remove_ships,place_ships};

pub(crate) fn repair(game_state: &GameState, target_system: u8, build_type: BuildType) -> GameState {
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
                let ships = remove_ships(&ships, &current_player, 0, 1);
                let ships = place_ships(&ships, &current_player, 1, 0);
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
