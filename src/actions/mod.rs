use std::collections::HashMap;

use itertools::Itertools;

use crate::data::game_state::{Action, ActionType, BasicAction, Color, GameState, PlayerArea, ResourceSlot, ResourceType, TurnState};

mod building;
mod tricktaking;
mod ending;
mod repairing;
mod influencing;
mod battling;
mod securing;
mod taxing;
pub mod moving;

fn use_action_pip(game_state: &GameState) -> GameState {
    match &game_state.turn_state {
        TurnState::Actions { action_type, pips_left } => {
            GameState {
                turn_state: TurnState::Actions { action_type: action_type.clone(), pips_left: pips_left - 1 },
                ..game_state.clone()}
            },
            _ => panic!("Cannot use action pip in {:?}", game_state.turn_state)
    }
}

fn execute_prelude_action(game_state: &GameState, action: BasicAction, resource: Option<ResourceType>) -> GameState {
    match (action.clone(), resource.clone().expect("No Resource in ResourceSlot"), game_state.lead_card.clone().unwrap().0.action_type) {
        (BasicAction::Build { target_system, build_type }, ResourceType::Material, _) => building::build(game_state, target_system, build_type),
        (BasicAction::Build { target_system, build_type }, ResourceType::Psionics, ActionType::Construction) => building::build(game_state, target_system, build_type),
        (BasicAction::Repair { target_system, build_type }, ResourceType::Material, _) => repairing::repair(game_state, target_system, build_type),
        (BasicAction::Repair { target_system, build_type }, ResourceType::Psionics, ActionType::Mobilization) => repairing::repair(game_state, target_system, build_type),
        (BasicAction::Tax { target_system, target_player }, ResourceType::Psionics, ActionType::Administration) => taxing::tax(game_state, target_system, target_player),
        (BasicAction::Influence { card_id }, ResourceType::Psionics, ActionType::Administration | ActionType::Mobilization) => influencing::influence(game_state, card_id),
        (BasicAction::Move { origin_id, destination_id, fresh_ships, damaged_ships }, ResourceType::Fuel, _) => moving::move_ships(game_state, origin_id, destination_id, fresh_ships, damaged_ships),
        (BasicAction::Move { origin_id, destination_id, fresh_ships, damaged_ships }, ResourceType::Psionics, ActionType::Agression | ActionType::Mobilization) => moving::move_ships(game_state, origin_id, destination_id, fresh_ships, damaged_ships),
        (BasicAction::Catapult { origin_system, destination_systems }, ResourceType::Fuel, _) => moving::catapult(game_state, origin_system, destination_systems),
        (BasicAction::Catapult { origin_system, destination_systems }, ResourceType::Psionics, ActionType::Agression | ActionType::Mobilization) => moving::catapult(game_state, origin_system, destination_systems),
        (BasicAction::Secure { card_id, vox_payload }, ResourceType::Relics, _) => securing::secure(game_state, card_id, vox_payload),
        (BasicAction::Secure { card_id, vox_payload }, ResourceType::Psionics, ActionType::Agression) => securing::secure(game_state, card_id, vox_payload),
        (BasicAction::Battle { target_system, target_player, dice }, ResourceType::Psionics, ActionType::Agression) => battling::battle(game_state, target_system, target_player, dice),
        _ => panic!("Cannot execute {:?} with {:?} resource and {:?} lead", action.clone(), resource.clone(), game_state.lead_card.clone().unwrap().0.action_type)
    }
}

fn allocate_resources(game_state: &GameState, configuration: Vec<(u8, ResourceType)>) -> GameState {
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
        .. game_state.clone()
    }
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
                Action::PlayLeadCard { card, declare } => tricktaking::play_lead_card(game_state, card, declare),
                Action::Surpass { card, seize } => tricktaking::surpass(game_state, card, seize),
                Action::Copy { card, seize } => tricktaking::copy(game_state, card, seize),
                Action::Pivot { card, seize } => tricktaking::pivot(game_state, card, seize),
                _ => panic!("Can only Execute TrickTaking Actions")
            }
        },
        TurnState::Prelude { action_type, pips_left } => {
            match action {
                Action::EndPrelude => {
                    let mut new_game_state = game_state.clone();
                    new_game_state.turn_state = TurnState::Actions { action_type: action_type.clone(), pips_left: pips_left.clone() };
                    new_game_state
                },
                Action::PreludeResourceAction { basic_action, used_resource } => execute_prelude_action(
                    game_state,
                    basic_action,
                    game_state.players.get(&game_state.current_player).unwrap().get_resource(used_resource)
                ),
                Action::UseWeapons { used_resource } => todo!(),
                Action::PreludeCard { guild_card, prelude_action_payload } => todo!(),
                _ => panic!("Cannot execute {:?} in Prelude", action)
            }
        },
        TurnState::Actions { action_type, pips_left } => {
            if action == Action::EndTurn {return ending::end_turn(game_state)}
            if *pips_left == 0 {panic!("No Action pips left in {:?}, when executing {:?}", game_state.turn_state, action)}
            return match (action_type, action) {
                (ActionType::Administration, Action::MainAction { basic_action }) => match basic_action {
                    BasicAction::Repair { target_system, build_type } => repairing::repair(&use_action_pip(&game_state), target_system, build_type),
                    BasicAction::Tax { target_system, target_player } => taxing::tax(&use_action_pip(&game_state), target_system, target_player),
                    BasicAction::Influence { card_id } => influencing::influence(&use_action_pip(&game_state), card_id),
                    _ => panic!("Cannot execute Action with Administration Action Card")                    
                },
                (ActionType::Agression, Action::MainAction { basic_action }) => match basic_action {
                    BasicAction::Move { origin_id, destination_id, fresh_ships, damaged_ships } => moving::move_ships(&use_action_pip(&game_state), origin_id, destination_id, fresh_ships, damaged_ships),
                    BasicAction::Catapult { origin_system, destination_systems } => moving::catapult(game_state, origin_system, destination_systems),
                    BasicAction::Secure { card_id , vox_payload} => securing::secure(&use_action_pip(&game_state), card_id, vox_payload),
                    BasicAction::Battle { target_system, target_player, dice } => battling::battle(&use_action_pip(&game_state), target_system, target_player, dice),
                    _ => panic!("Cannot execute Action with Aggresion Action Card")
                },
                (ActionType::Construction, Action::MainAction { basic_action }) => match basic_action {
                    BasicAction::Build { target_system, build_type } => building::build(&use_action_pip(game_state), target_system, build_type),
                    BasicAction::Repair { target_system, build_type } => repairing::repair(&use_action_pip(&game_state), target_system, build_type),
                    _ => panic!("Cannot execute Action with Aggresion Action Card")                    
                },
                (ActionType::Mobilization, Action::MainAction { basic_action }) => match basic_action {
                    BasicAction::Move { origin_id, destination_id, fresh_ships, damaged_ships } => moving::move_ships(&use_action_pip(&game_state), origin_id, destination_id, fresh_ships, damaged_ships),
                    BasicAction::Catapult { origin_system, destination_systems } => moving::catapult(game_state, origin_system, destination_systems),
                    BasicAction::Influence { card_id } => influencing::influence(&use_action_pip(&game_state), card_id),
                    _ => panic!("Cannot execute Action with Mobilization Action Card")                    
                },
                (_ , Action::EndTurn) => todo!(),
                _ => panic!("")
            }
        }
        TurnState::AllocateResources { ..} => match action {
            Action::AllocateResources { configuration } => allocate_resources(game_state, configuration),
            _ => panic!("Can only AllocateResources in the AllocateResources Turnstate not {:?}", action)
        },
        TurnState::AllocateDiceResults { target_system, target_player, self_hits, hits, building_hits, keys } => todo!(),
    }
}