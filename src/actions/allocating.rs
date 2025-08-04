use std::collections::HashMap;

use itertools::Itertools;

use crate::data::game_state::{Color, GameState, PlayerArea, ResourceSlot, ResourceType, TurnState, HitTarget, RaidTarget};

pub (crate) fn allocate_resources(game_state: &GameState, configuration: Vec<(u8, ResourceType)>) -> GameState {
    let (current_player, additional_resources) = match &game_state.turn_state {
        TurnState::AllocateResources { player, resources } => (player, resources),
        _ => panic!("Cannot allocate resources in {:?}", game_state.turn_state)
    };
    let current_resource_slots = game_state.players.get(&current_player).unwrap().resource_slots.clone();

    let available_resources: Vec<ResourceType> = current_resource_slots
        .iter()
        .filter_map(|r| {
                    match r {
                        ResourceSlot::Used { keys: _, resource } => Some(resource.clone()),
                        _ => None
                    }
                })
                .chain(additional_resources.iter().cloned())
                .collect();

    let available_resourceslots = current_resource_slots
        .iter()
        .filter(|s| match s {
            ResourceSlot::Used {..} | ResourceSlot::Unused {..} => true,
            _ => false 
        })
        .count();

    if configuration.len() > available_resourceslots {panic!("Cannot allocate {:?} resources into {:?} available ResourceSlots", configuration.len(), available_resourceslots)}

    if configuration.iter().map(|(i,_)|i).tuple_windows().any(|(i1,i2)| i1==i2) {panic!("Configuration includes index duplicates")}

    let config_resources = configuration
        .iter()
        .map(|(_,r)| r)
        .fold(
            vec![
            (ResourceType::Fuel, 0),
            (ResourceType::Material, 0),
            (ResourceType::Weapons, 0),
            (ResourceType::Relics, 0),
            (ResourceType::Psionics, 0),
            ]
            .into_iter()
            .collect::<HashMap<ResourceType, u8>>(),
            |mut acc, r| {
            *acc.entry(r.clone()).or_insert(0) += 1;
            acc
            },
        );
        
    let available_resources = available_resources
        .iter()
        .fold(
            vec![
            (ResourceType::Fuel, 0),
            (ResourceType::Material, 0),
            (ResourceType::Weapons, 0),
            (ResourceType::Relics, 0),
            (ResourceType::Psionics, 0),
            ]
            .into_iter()
            .collect::<HashMap<ResourceType, u8>>(),
            |mut acc, r| {
            *acc.entry(r.clone()).or_insert(0) += 1;
            acc
            },
        );

    if config_resources.iter().any(|(resource, &config_count)| config_count > *available_resources.get(resource).unwrap()) {
        panic!("Trying to allocate more resources than available: config: {:?}, available: {:?}", config_resources, available_resources);
    }

    let new_resource_slots: Vec<ResourceSlot> = current_resource_slots
        .iter()
        .zip(
            (0..current_resource_slots.len()).map(|i| {
                configuration.iter()
                    .find(|(idx, _)| *idx as usize == i)
                    .map(|(_, r)| r.clone())
            })
        )
        .map(|(s,r)| {
            match (s,r) {
                (ResourceSlot::Used { keys, ..} | ResourceSlot::Unused { keys } , None) => ResourceSlot::Unused { keys: *keys },
                (ResourceSlot::Used { keys, ..} | ResourceSlot::Unused { keys }, Some(resource)) => ResourceSlot::Used { keys: *keys, resource: resource },
                (ResourceSlot::Covered { keys }, None) => ResourceSlot::Covered { keys: *keys },
                (ResourceSlot::Covered { ..}, Some(_)) => panic!("Cannot allocate Resource in Covered ResourceSlot"),
            }
        })
        .collect();

    let new_players: HashMap<Color, PlayerArea> = game_state.players
            .iter()
            .map(|(c,p)|if c == current_player {(c.clone(),PlayerArea{resource_slots: new_resource_slots.clone(), ..p.clone()})} else {(c.clone(),p.clone())})
            .collect();

    let overflow_resources: Vec<ResourceType> = available_resources.iter()
        .flat_map(|(resource, &available_count)| {
            let used_count = config_resources.get(resource).cloned().unwrap_or(0);
            if available_count > used_count {
                std::iter::repeat(resource.clone()).take((available_count - used_count) as usize).collect::<Vec<_>>()
            } else {
                Vec::new()
            }
        })
        .collect();

    let new_resource_reserve: HashMap<ResourceType,u8> = game_state.resource_reserve
        .iter()
        .map(|(r,c)|{
            let overflow = overflow_resources.iter().filter(|res| res == &r).count();
            (r.clone(), c+overflow as u8)
        })
        .collect();

    GameState {
        players: new_players,
        resource_reserve: new_resource_reserve,
        turn_state: game_state.next_turn_states[0].clone(),
        next_turn_states: game_state.next_turn_states[1..].to_vec(),
        .. game_state.clone()
    }
}

pub (crate) fn allocate_dice_results(game_state: &GameState, target_system: u8, target_player: Color, self_hits: u8, hits: u8, building_hits: u8, keys: u8 , own_hits: Vec<HitTarget>, other_hits: Vec<HitTarget>, raid_target: Vec<RaidTarget>) -> GameState{
    let (system_type, building_slots, ships) = match game_state.systems[target_system as usize].clone() {
        crate::data::system::System::Unused => panic!("Cannot allocate in unused System"),
        crate::data::system::System::Used { system_id, system_type, building_slots, ships, controlled_by, connects_to } => (system_type, building_slots, ships),
    };
    
    let hit_sum: u8 = own_hits.iter().map(|h| match h {
        HitTarget::Damaged => 1,
        HitTarget::Fresh => 1,
        HitTarget::Destroyed => 2,
        HitTarget::Building { .. } => panic!("Cannot hit own buildings, when attacking"),
    }).sum();

    let possible_hits = {
        let s = ships.get(&game_state.current_player).unwrap();
        s.damaged + 2*s.fresh
    };


    if hit_sum < self_hits {panic!("Need to allocate all")}

    game_state.clone()
}