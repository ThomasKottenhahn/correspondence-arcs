#[cfg(test)]
mod test{
    use correspondence_arcs::data;
    use correspondence_arcs::board;
    use correspondence_arcs::actions;
    use correspondence_arcs::setup_cards::two_player_frontiers;

    #[test]
    fn test_building_ships(){
        let test_setup: data::SetupCard = two_player_frontiers();
        let game_state: data::GameState = board::setup_game(&test_setup);

        let target_system = 17;
        let build_type = data::BuildType::Ship;

        let new_game_state = actions::build(&game_state, target_system, build_type);

        match &new_game_state.systems[target_system as usize] {
            data::System::Used {ships, controlled_by, ..} => {
                assert_eq!(ships, &vec![data::Ships { player: data::Color::Red, fresh: 4, damaged: 0 }, data::Ships { player: data::Color::Blue, fresh: 0, damaged: 0 }]);
                assert_eq!(controlled_by, &Some(data::Color::Red));
            }
            _ => panic!("Expected Used system variant")
        }
    }

    #[test]
    fn test_execute_building_action(){
        let test_setup: data::SetupCard = two_player_frontiers();
        let game_state: data::GameState = board::setup_game(&test_setup);

        let target_system = 17;
        let build_type = data::BuildType::Ship;

        let new_game_state = actions::execute_action(&game_state, data::Action::Build{target_system, build_type});

        match &new_game_state.systems[target_system as usize] {
            data::System::Used {ships, controlled_by, ..} => {
                assert_eq!(ships, &vec![data::Ships { player: data::Color::Red, fresh: 4, damaged: 0 }, data::Ships { player: data::Color::Blue, fresh: 0, damaged: 0 }]);
                assert_eq!(controlled_by, &Some(data::Color::Red));
            }
            _ => panic!("Expected Used system variant")
        }
    }

    #[test]
    fn test_building_city(){
        let test_setup: data::SetupCard = data::SetupCard { players: 2, cluster_out_of_play: vec![0,5], a_locations: vec![20,11], b_locations: vec![17, 18], c_locations: vec![2,4,14,15]};
        let game_state: data::GameState = board::setup_game(&test_setup);

        let target_system = 16;
        let build_type = data::BuildType::City;

        let g1 = actions::execute_action(&game_state,data::Action::Move { origin_id: 17, destination_id: 16, fresh_ships: 2, damaged_ships: 0 });
        let new_game_state = actions::execute_action(&g1, data::Action::Build{target_system, build_type});

        match &new_game_state.systems[target_system as usize] {
            data::System::Used {building_slots, controlled_by, ..} => {
                assert_eq!(building_slots, &vec![data::BuildingSlot::Occupied { fresh: true, player: data::Color::Red, building_type: data::BuildingType::City}, data::BuildingSlot::Empty]);
                assert_eq!(controlled_by, &Some(data::Color::Red));
            }
            _ => panic!("Expected Used system variant")
        }
    }

    #[test]
    fn building_starport(){
        let test_setup: data::SetupCard = two_player_frontiers();
        let game_state: data::GameState = board::setup_game(&test_setup);

        let target_system: u8 = 15;
        let build_type = data::BuildType::Starport;

        let g1 = actions::execute_action(&game_state,data::Action::Move { origin_id: 17, destination_id: 16, fresh_ships: 2, damaged_ships: 0 });
        let g2 = actions::execute_action(&g1,data::Action::Move { origin_id: 16, destination_id: 15, fresh_ships: 2, damaged_ships: 0 });
        let new_game_state = actions::execute_action(&g2, data::Action::Build{target_system, build_type});

        match &new_game_state.systems[target_system as usize] {
            data::System::Used {building_slots, controlled_by, ..} => {
                assert_eq!(building_slots, &vec![data::BuildingSlot::Occupied { fresh: true, player: data::Color::Red, building_type: data::BuildingType::Starport}, data::BuildingSlot::Empty]);
                //Shoulf be controlled by no one, because of two blue ships
                assert_eq!(controlled_by, &None);
            }
            _ => panic!("Expected Used system variant")
        }
    }

    #[test]
    #[should_panic(expected = "No building slots available")]
    fn building_in_gate(){
        let test_setup: data::SetupCard = two_player_frontiers();
        let game_state: data::GameState = board::setup_game(&test_setup);

        let target_system = 3;
        let build_type = data::BuildType::Starport;

        let g1 = actions::execute_action(&game_state,data::Action::Move { origin_id: 17, destination_id: 3, fresh_ships: 2, damaged_ships: 0 });
        let _ = actions::execute_action(&g1, data::Action::Build{target_system, build_type});
    }

    #[test]
    #[should_panic(expected = "All building slots are occupied")]
    fn build_in_full_system(){
        let test_setup: data::SetupCard = two_player_frontiers();
        let game_state: data::GameState = board::setup_game(&test_setup);

        let target_system = 17;
        let build_type = data::BuildType::Starport;

        print!("{:?}", game_state.systems[17]);
        let _ = actions::execute_action(&game_state, data::Action::Build{target_system, build_type: build_type.clone()});
    }
}