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

        let new_game_state = actions::execute_actions(&game_state, vec![
            data::Action::PlayLeadCard { card: data::ActionCard { action_type: data::ActionType::Construction, number: 2, pips: 4, declared_ambition: Some(data::AmbitionTypes::Tycoon) }, declare: None },
            data::Action::EndPrelude,
            data::Action::Build { target_system: target_system, build_type: build_type }
        ]);

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

        let new_game_state = actions::execute_actions(&game_state, vec![
            data::Action::PlayLeadCard { card: data::ActionCard { action_type: data::ActionType::Construction, number: 2, pips: 4, declared_ambition: Some(data::AmbitionTypes::Tycoon) }, declare: None },
            data::Action::EndPrelude,
            data::Action::Build { target_system: target_system, build_type: build_type }
        ]);

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

        let game_state = actions::move_ships(&game_state, 17, 16, 2, 0);

        let new_game_state = actions::execute_actions(&game_state, vec![
            data::Action::PlayLeadCard { card: data::ActionCard { action_type: data::ActionType::Construction, number: 2, pips: 4, declared_ambition: Some(data::AmbitionTypes::Tycoon) }, declare: None },
            data::Action::EndPrelude,
            data::Action::Build { target_system: target_system, build_type: build_type }
        ]);      

        match &new_game_state.systems[target_system as usize] {
            data::System::Used {building_slots, controlled_by, ..} => {
                assert_eq!(building_slots, &vec![data::BuildingSlot::Occupied { fresh: true, player: data::Color::Red, building_type: data::BuildingType::City, used: false}, data::BuildingSlot::Empty]);
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

        let g1 = actions::move_ships(&game_state,17, 16, 2, 0);
        let g2 = actions::move_ships(&g1,16,15, 2, 0);
        
        let new_game_state = actions::execute_actions(&g2, vec![
            data::Action::PlayLeadCard { card: data::ActionCard { action_type: data::ActionType::Construction, number: 2, pips: 4, declared_ambition: Some(data::AmbitionTypes::Tycoon) }, declare: None },
            data::Action::EndPrelude,
            data::Action::Build { target_system: target_system, build_type: build_type }
        ]);

        match &new_game_state.systems[target_system as usize] {
            data::System::Used {building_slots, controlled_by, ..} => {
                assert_eq!(building_slots, &vec![data::BuildingSlot::Occupied { fresh: true, player: data::Color::Red, building_type: data::BuildingType::Starport, used: false}, data::BuildingSlot::Empty]);
                //Should be controlled by no one, because of two blue ships
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

        let g1 = actions::move_ships(&game_state, 17, 3, 2, 0);
        let _ = actions::execute_actions(&g1, vec![
            data::Action::PlayLeadCard { card: data::ActionCard { action_type: data::ActionType::Construction, number: 2, pips: 4, declared_ambition: Some(data::AmbitionTypes::Tycoon) }, declare: None },
            data::Action::EndPrelude,
            data::Action::Build { target_system: target_system, build_type: build_type }
        ]);
    }

    #[test]
    #[should_panic(expected = "All building slots are occupied")]
    fn build_in_full_system(){
        let test_setup: data::SetupCard = two_player_frontiers();
        let game_state: data::GameState = board::setup_game(&test_setup);

        let target_system = 17;
        let build_type = data::BuildType::Starport;

        let _ = actions::execute_actions(&game_state, vec![
            data::Action::PlayLeadCard { card: data::ActionCard { action_type: data::ActionType::Construction, number: 2, pips: 4, declared_ambition: Some(data::AmbitionTypes::Tycoon) }, declare: None },
            data::Action::EndPrelude,
            data::Action::Build { target_system: target_system, build_type: build_type }
        ]);
    }
}