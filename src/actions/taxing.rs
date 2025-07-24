use crate::data::game_state::{GameState, Color, Trophy, ReserveType, TurnState};
use crate::data::system::{BuildingType, System, SystemType};

pub(crate) fn tax(game_state: &GameState, target_system: u8, target_player: Color) -> GameState {
    let system = &game_state.systems[target_system as usize];
    let tax_rival = target_player != game_state.current_player;

    let mut new_game_state =  game_state.clone();

    match system {
        System::Unused => panic!("Cannot tax Unused System"),
        System::Used {system_type, controlled_by, .. } => {
            if tax_rival {
                if controlled_by != &Some(game_state.current_player.clone()) {
                    panic!("Cannot tax a rival in a System controlled by another player");
                }
                let rivals_play_area = new_game_state.get_player_area(&target_player);
                let current_player_area = new_game_state.get_player_area(&game_state.current_player);
                current_player_area.add_trophies(vec![Trophy {
                    trophy_type: ReserveType::Agents,
                    count: 1,
                    player: target_player.clone(),
                }]);
                new_game_state.players.insert(target_player.clone(), rivals_play_area);
                new_game_state.update_players_reserve(&target_player, &ReserveType::Agents, -1);
            }
            new_game_state.systems[target_system as usize] = new_game_state.systems[target_system as usize].use_building(&BuildingType::City, &target_player);
            
            let taxed_resource = match system_type {
                SystemType::Gate => panic!("Cannot tax Gate System"),
                SystemType::Planet { resource } => resource.clone(),
            };

            let resource_count = new_game_state.resource_reserve.get(&taxed_resource).expect("No Resource in Reserve").clone();
            if resource_count > 0 {
                new_game_state.resource_reserve.insert(taxed_resource.clone(), resource_count - 1);
                new_game_state.next_turn_states = vec![new_game_state.turn_state];
                new_game_state.turn_state = TurnState::AllocateResources {player: new_game_state.current_player.clone(), resources: vec![taxed_resource] };
            }
        }

    }
    new_game_state
}
