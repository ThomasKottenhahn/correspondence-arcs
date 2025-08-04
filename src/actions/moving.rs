use crate::board::{place_ships, remove_ships};
use crate::data::game_state::{Color, GameState};
use crate::data::system::{BuildingSlot, BuildingType, System, SystemType};

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
            if !connects_to.contains(&destination_system_id) {panic!("Destination not connected to Origin")}
            let updated_ships = remove_ships(&ships, &game_state.current_player, fresh, damaged);
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
            let updated_ships = place_ships(&ships, &game_state.current_player, fresh, damaged);
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

fn bfs_search_systems(systems: &Vec<System>, start_system_id: u8, color: &Color) -> Vec<u8> {
    if match &systems[(start_system_id.clone()/3) as usize] {
        System::Used { system_type: SystemType::Gate, controlled_by, .. } => controlled_by == &None || controlled_by == &Some(color.clone()),
        _ => false
    } {return vec![start_system_id/8 as u8];}
    let mut stack = vec![start_system_id/3 as u8];
    let mut reachable: Vec<u8> = vec![];
    while stack.len() != 0 {
        let current = stack.pop().unwrap();
        match &systems[current as usize] {
            System::Unused => panic!("Cannot expand from unsused system"),
            System::Used { connects_to, .. } => connects_to.iter()
                .filter_map(|&i| match &systems[i as usize] {
                    System::Unused => None,
                    System::Used { system_type, controlled_by, .. } => {
                        Some((i, system_type, controlled_by))
                    },
                }),
        }.for_each(|(i, st, c)| {
            reachable.push(i);
            if st == &SystemType::Gate && (c == &None || c == &Some(color.clone())) {stack.push(i);}
        });
    }
    return reachable;
}

pub(crate) fn catapult(game_state: &GameState, origin_system: u8, destination_systems: Vec<(u8,u8,u8)>) -> GameState {
    let current_player = game_state.current_player.clone();

    let (building_slots, ships) = match &game_state.systems[origin_system as usize] {
        System::Unused => panic!("Cannot catapult from Unused System"),
        System::Used {building_slots, ships, ..} => (building_slots,ships.get(&current_player).unwrap())
    };

    let has_loyal_starport = building_slots.iter().any(|b| match b {
            BuildingSlot::Occupied { player, building_type: BuildingType::Starport, .. } if player == &current_player => true,
            _ => false
        });
    if !has_loyal_starport {panic!("Cannot catapult from {:?}, because the system has no loyal Starport", origin_system)}
    
    //Check if we move less or equal to the ships present
    let (fresh, damaged) = destination_systems
        .iter()
        .fold((0, 0), |(fresh_sum, damaged_sum), &(_, fresh, damaged)| {
            (fresh_sum + fresh, damaged_sum + damaged)
        });

    if fresh > ships.fresh || damaged > ships.damaged {panic!("Catapulting {:?} fresh and {:?} damaged Ships when only {:?} fresh and {:?} damaged are present", fresh, damaged, ships.fresh, ships.damaged)}

    
    let systems: Vec<System>  = destination_systems.iter()
        .map(|(sys, _, _)| game_state.systems[*sys as usize].clone())
        .collect();

    //Check only one destination is a Planet System
    if systems.iter().filter(|s| match s {
        System::Used { system_type: SystemType::Planet { .. }, .. } => true,
        _ => false
    }).count() > 1 {panic!("Cannot catapult into more than one Planet System")}
    
    let reachable = bfs_search_systems(&game_state.systems, origin_system, &current_player);
    //Check all can be reached without entering a planet system or rival controlled System
    if destination_systems.iter().any(|(s, _, _)| !reachable.contains(s)) {panic!("Cannot catapult into unreachable (because of lacking control) System")}

    return destination_systems.iter()
        .fold(game_state.clone(), |g, (sys, fresh, damaged)| move_ships(&g, origin_system, *sys, *fresh, *damaged))
}
