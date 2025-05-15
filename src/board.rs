use crate::data::GameState;
use crate::data::PlayerArea;
use crate::data::TurnState;
use crate::data::SetupCard;
use crate::data::System;
use crate::data::Color;
use crate::data::SystemType;
use crate::data::BuildingSlot;
use crate::data::ResourceType;


fn create_reach(setup_card: &SetupCard) -> Vec<System> {
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
                ships: vec![],
                controlled_by: None,
                connects_to: connections
            }
        }
        };
        systems.push(gate);
    }
    
    // Resources and number of building Slots on each Planet
    let resource_types = vec![ResourceType::Weapons, ResourceType::Fuel, ResourceType::Material, ResourceType::Psionics, ResourceType::Weapons, ResourceType::Relics, ResourceType::Material, ResourceType::Fuel, ResourceType::Weapons, ResourceType::Relics, ResourceType::Fuel, ResourceType::Material, ResourceType::Weapons, ResourceType::Relics, ResourceType::Psionics, ResourceType::Material, ResourceType::Fuel, ResourceType::Psionics];
    let building_slots_nr = vec![2,1,2,1,1,2,1,1,2,2,2,1,1,1,1,1,2,1];
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
                if (i == 5 || i == 14) && !setup_card.cluster_out_of_play.contains(&((i/3)as u8)) {connections.push((i+7) as u8)}
                if (i == 6 || i == 15) && !setup_card.cluster_out_of_play.contains(&((i/3)as u8)) {connections.push((i+5) as u8)}
                
                System::Used {
                    system_id: (6+i) as u8,
                    system_type: SystemType::Planet {resource: resource_types[i].clone()},
                    building_slots: systems_building_slots,
                    ships: vec![],
                    controlled_by: None,
                    connects_to: connections
                }
        
            }
        };
        systems.push(planet);
    }

    return systems;
}

fn setup_player_area(player_color: &Color, initiative: bool) -> PlayerArea{
    return PlayerArea{
        player: player_color.clone(),
        initiative: initiative,
        action_cards: vec![],
        controlled_systems: vec![],
        controlled_ships: vec![],
        controlled_buildings: vec![],
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
    let players: Vec<PlayerArea> = vec![Color::Red, Color::Blue].iter().map(|x|setup_player_area(x, false)).collect();
    return GameState{
        players: players,
        current_player: Color::Red,
        turn_state: TurnState::TrickTaking,
        chapter: 1,
        systems: create_reach(setup_card),
        court: vec![],
        court_discard_pile: vec![],
        court_draw_pile: vec![],
        action_discard: vec![],
        lead_card: None,
        follow_cards: vec![]        
    };
}

pub fn update_control(system: &System) -> System{
    match system {
        System::Unused => system.clone(),
        System::Used {
            system_id,
            system_type,
            building_slots,
            ships,
            connects_to, ..
        } => {
            let new_controlled_by = {
                if ships.len() == 0 {
                    None
                } else {
                    let mut player = ships[0].player.clone();
                    let mut most = ships[0].fresh.clone();
                    for i in 1..ships.len() {
                        if ships[i].fresh > most {
                            player = ships[i].player.clone();
                            most = ships[i].fresh.clone();
                        } else if ships[i].fresh == most {
                            player = Color::None;
                        }
                    }
                    if player == Color::None {
                        None
                    } else {
                        Some(player.clone())
                    }
                }
            };
            return System::Used {
                system_id: system_id.clone(),
                system_type: system_type.clone(),
                building_slots: building_slots.clone(),
                ships: ships.clone(),
                controlled_by: new_controlled_by,
                connects_to: connects_to.clone()
            };
        }
    }
}