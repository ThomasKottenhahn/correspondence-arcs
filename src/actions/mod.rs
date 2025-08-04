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
mod allocating;
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
            Action::AllocateResources { configuration } => allocating::allocate_resources(game_state, configuration),
            _ => panic!("Can only AllocateResources in the AllocateResources Turnstate not {:?}", action)
        },
        TurnState::AllocateDiceResults { target_system, target_player, self_hits, hits, building_hits, keys } => todo!(),
    }
}