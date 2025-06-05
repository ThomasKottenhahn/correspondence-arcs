use std::collections::hash_map;

use crate::data::system::{System, SystemType, Ships, BuildingSlot, BuildingType};
use crate::data::setup_cards::{SetupCard};
use crate::data::game_state::{GameState, PlayerArea, TurnState, Color, ResourceType};

use crate::actions::place_ships;
use crate::actions::place_building;

fn create_reach(setup_card: &SetupCard) -> Vec<System> {
    let all_colors: Vec<Color> = vec![Color::Red, Color::Blue, Color::White, Color::Yellow];
    let empty_ships: Vec<Ships> = all_colors[0..(setup_card.players as usize)].iter().map(|x| Ships{player: x.clone(), fresh: 0, damaged: 0}).collect();

    let mut systems:Vec<System> = vec![];
    // Create Gates
    for i in 0..6 {
        let gate ={
            let mut connections = vec![];
            // Add Gate before
            let mut j = 5;
            while setup_card.cluster_out_of_play.contains(&((i+j)%6)){
                j = j-1
            }
            connections.push((i+j)%6);

            // Add Gate after
            let mut j = 1;
            while setup_card.cluster_out_of_play.contains(&((i+j)%6)){
                j = j+1
            }
            connections.push((i+j)%6);

            if setup_card.cluster_out_of_play.contains(&i) {
                System::Unused
            }else{
                System::Used {
                system_id: i,
                system_type: SystemType::Gate,
                building_slots: vec![],
                ships: empty_ships.clone(),
                controlled_by: None,
                connects_to: connections
            }
        }
        };
        systems.push(gate);
    }
    
    // Resources and number of building Slots on each Planet
    let resource_types = vec![ResourceType::Weapons, ResourceType::Fuel, ResourceType::Material, ResourceType::Psionics, ResourceType::Weapons, ResourceType::Relics, ResourceType::Material, ResourceType::Fuel, ResourceType::Weapons, ResourceType::Relics, ResourceType::Fuel, ResourceType::Material, ResourceType::Weapons, ResourceType::Relics, ResourceType::Psionics, ResourceType::Material, ResourceType::Fuel, ResourceType::Psionics];
    let building_slots_nr = vec![2,1,2,1,1,2,1,1,2,2,2,1,1,1,2,1,2,1];
    let empty_building_slot = BuildingSlot::Empty;

    // Create Planets
    for i in 0..18{
        let planet = {
            if setup_card.cluster_out_of_play.contains(&(((i/3)as u8))){
                System::Unused
            }else{
                let systems_building_slots = {
                    if building_slots_nr[i] == 1{
                        vec![empty_building_slot.clone()]
                    } else{
                        vec![empty_building_slot.clone(), empty_building_slot.clone()]
                    }
                };
                
                // Gate connection
                let mut connections = vec![(i/3) as u8];
                // adjacent Planets
                if i != 0 {if i/3 == (i-1)/3 {connections.push((i+5) as u8)}}
                if i/3 == (i+1)/3 {connections.push((i+7) as u8)}
                
                // Special Borders
                if (i == 5 || i == 14) && !setup_card.cluster_out_of_play.contains(&(((i+1)/3)as u8)) {connections.push((i+7) as u8)}
                if (i == 6 || i == 15) && !setup_card.cluster_out_of_play.contains(&(((i-1)/3)as u8)) {connections.push((i+5) as u8)}
                
                System::Used {
                    system_id: (6+i) as u8,
                    system_type: SystemType::Planet {resource: resource_types[i].clone()},
                    building_slots: systems_building_slots,
                    ships: empty_ships.clone(),
                    controlled_by: None,
                    connects_to: connections
                }
        
            }
        };
        systems.push(planet);
    }

    return systems;
}

fn setup_player_area(player_color: &Color) -> PlayerArea {
    return PlayerArea{
        player: player_color.clone(),
        initiative: false,
        action_cards: vec![],
        guild_cards: vec![],
        reserve_ships: 15,
        reserve_agents: 10,
        reserve_starports: 5,
        reserve_cities: 5,
        // TODO: resourceSlots
        resource_slots: vec![],
        captives: vec![],
        tropies: vec![]
    }
}

pub fn setup_game(setup_card: &SetupCard) -> GameState {
    let all_colors: Vec<Color> = vec![Color::Red, Color::Blue, Color::White, Color::Yellow].iter().take(setup_card.players.into()).cloned().collect();
    let mut players: Vec<PlayerArea> = all_colors[0..(setup_card.players as usize)].iter().map(|x|setup_player_area(x)).collect();
    players[0].initiative = true;

    let systems = create_reach(setup_card);
    let systems = setup_card
        .a_locations
        .iter()
        .zip(all_colors.iter())
        .fold(systems, |mut acc, (&system_id, color)| {
            let idx = system_id as usize;
            let (left, right) = acc.split_at_mut(idx);
            let system = &mut right[0];
            match system{
                System::Used { 
                    system_id,
                    system_type,
                    building_slots,
                    ships,
                    controlled_by,
                    connects_to 
                } => *system = System::Used { 
                    system_id: *system_id,
                    system_type: system_type.clone(),
                    building_slots: place_building(&building_slots, BuildingSlot::Occupied { fresh: true, player: color.clone(), building_type: BuildingType::City , used: false}),
                    ships: place_ships(&ships, color.clone(), 3, 0),
                    controlled_by: controlled_by.clone(),
                    connects_to: connects_to.to_vec()
                },
                System::Unused => panic!("Cannot setup in unused System")
            };
            acc
        });

    let systems = setup_card
        .b_locations
        .iter()
        .zip(all_colors.iter())
        .fold(systems, |mut acc, (&system_id, color)| {
            let idx = system_id as usize;
            let (left, right) = acc.split_at_mut(idx);
            let system = &mut right[0];
            match system{
                System::Used { 
                    system_id,
                    system_type,
                    building_slots,
                    ships,
                    controlled_by,
                    connects_to 
                } => *system = System::Used { 
                    system_id: *system_id,
                    system_type: system_type.clone(),
                    building_slots: place_building(&building_slots, BuildingSlot::Occupied { fresh: true, player: color.clone(), building_type: BuildingType::Starport , used: false}),
                    ships: place_ships(&ships, color.clone(), 3, 0),
                    controlled_by: controlled_by.clone(),
                    connects_to: connects_to.to_vec()
                },
                System::Unused => panic!("Cannot setup in unused System")
            };
            acc
        });

    let systems = setup_card
        .c_locations
        .iter()
        .zip(all_colors.iter().chain(&all_colors))
        .fold(systems, |mut acc, (&system_id, color)| {
            let idx = system_id as usize;
            let (left, right) = acc.split_at_mut(idx);
            let system = &mut right[0];
            match system{
                System::Used { 
                    system_id,
                    system_type,
                    building_slots,
                    ships,
                    controlled_by,
                    connects_to 
                } => *system = System::Used { 
                    system_id: *system_id,
                    system_type: system_type.clone(),
                    building_slots: building_slots.clone(),
                    ships: place_ships(&ships, color.clone(), 2, 0),
                    controlled_by: controlled_by.clone(),
                    connects_to: connects_to.to_vec()
                },
                System::Unused => panic!("Cannot setup in unused System")
            };
            acc
        });

    let systems = systems.iter().map(|s| s.update_control()).collect();

    let mut resource_reserve: hash_map::HashMap<ResourceType, u8> = hash_map::HashMap::new();
    for (key, value) in vec![ResourceType::Weapons, ResourceType::Fuel, ResourceType::Material, ResourceType::Psionics, ResourceType::Relics].iter().map(|x| (x.clone(), 5 as u8)) {
        resource_reserve.insert(key.clone(), value);
    }

    return GameState{
        players: players.clone().into_iter().map(|p| (p.player.clone(), p)).collect(),
        current_player: Color::Red,
        players_in_round: players.len() as u8,
        turn_state: TurnState::TrickTaking,
        next_turn_state: None,
        initiative: Color::Red,
        seized: None,
        zero_marker: false,
        chapter: 1,
        systems: systems,
        resource_reserve: resource_reserve,
        court: vec![],
        court_discard_pile: vec![],
        court_draw_pile: vec![],
        action_discard: vec![],
        lead_card: None,
        follow_cards: vec![]        
    };
}