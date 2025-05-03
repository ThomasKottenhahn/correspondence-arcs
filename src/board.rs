use crate::data::SetupCard;
use crate::data::System;
use crate::data::SystemType;
use crate::data::BuildingSlot;
use crate::data::ResourceType;


pub fn create_reach(setup_card: &SetupCard) -> Vec<System> {
    let mut systems:Vec<System> = vec![];
    for i in 0..6 {
        let gate ={
            let mut connections = vec![];
            let mut j = 5;
            while setup_card.cluster_out_of_play.contains(&((i+j)%6)){
                j = j-1
            }
            connections.push((i+j)%6);

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
    let resource_types = vec![ResourceType::Weapons, ResourceType::Fuel, ResourceType::Material, ResourceType::Psionics, ResourceType::Weapons, ResourceType::Relics, ResourceType::Material, ResourceType::Fuel, ResourceType::Weapons, ResourceType::Relics, ResourceType::Fuel, ResourceType::Material, ResourceType::Weapons, ResourceType::Relics, ResourceType::Psionics, ResourceType::Material, ResourceType::Fuel, ResourceType::Psionics];
    let building_slots_nr = vec![2,1,2,1,1,2,1,1,2,2,2,1,1,1,1,1,2,1];
    let empty_building_slot = BuildingSlot::Empty;

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
                
                let mut connections = vec![(i/3) as u8];
                if i != 0 {if i/3 == (i-1)/3 {connections.push((i+5) as u8)}}
                if i/3 == (i+1)/3 {connections.push((i+7) as u8)}
                
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

    for i in 0..systems.len() {
        println!("{:?}",systems[i]);
    }
    return systems;
}