#[cfg(test)]
mod test {
    use correspondence_arcs::data;
    use correspondence_arcs::board;
    use correspondence_arcs::actions;
    use correspondence_arcs::setup_cards::two_player_frontiers;

    #[test]
    fn move_1_ship() {
        let test_setup: data::SetupCard = two_player_frontiers();
        let game_state: data::GameState = board::setup_game(&test_setup);

        let origin_system = 17;
        let destination_system = 3;

        let new_game_state = actions::execute_actions(&game_state, vec![
            data::Action::PlayLeadCard { card: data::ActionCard { action_type: data::ActionType::Mobilization, number: 2, pips: 4, declared_ambition: Some(data::AmbitionTypes::Tycoon) }, declare: None },
            data::Action::EndPrelude,
            data::Action::Move { origin_id: origin_system, destination_id: destination_system, fresh_ships: 1, damaged_ships: 0 }
        ]);

        match &new_game_state.systems[origin_system as usize] {
            data::System::Used {ships, controlled_by, ..} => {
                assert_eq!(ships, &vec![data::Ships { player: data::Color::Red, fresh: 2, damaged: 0 }, data::Ships { player: data::Color::Blue, fresh: 0, damaged: 0 }]);
                assert_eq!(controlled_by, &Some(data::Color::Red))
            }
            _ => panic!("Expected Used System Variant")
        }

        match &new_game_state.systems[destination_system as usize] {
            data::System::Used {ships, controlled_by, ..} => {
                assert_eq!(ships, &vec![data::Ships { player: data::Color::Red, fresh: 1, damaged: 0 }, data::Ships { player: data::Color::Blue, fresh: 0, damaged: 0 }]);
                assert_eq!(controlled_by, &Some(data::Color::Red))
            }
            _ => panic!("Expected Used System Variant")
        }

    }

    #[test]
    fn move_all_ships() {
        let test_setup: data::SetupCard = two_player_frontiers();
        let game_state: data::GameState = board::setup_game(&test_setup);

        let origin_system = 17;
        let destination_system = 16;

        let new_game_state = actions::execute_actions(&game_state, vec![
            data::Action::PlayLeadCard { card: data::ActionCard { action_type: data::ActionType::Mobilization, number: 2, pips: 4, declared_ambition: Some(data::AmbitionTypes::Tycoon) }, declare: None },
            data::Action::EndPrelude,
            data::Action::Move { origin_id: origin_system, destination_id: destination_system, fresh_ships: 3, damaged_ships: 0 }
        ]);

        match &new_game_state.systems[origin_system as usize] {
            data::System::Used {ships, controlled_by, ..} => {
                assert_eq!(ships, &vec![data::Ships { player: data::Color::Red, fresh: 0, damaged: 0 }, data::Ships { player: data::Color::Blue, fresh: 0, damaged: 0 }]);
                assert_eq!(controlled_by, &None)
            }
            _ => panic!("Expected Used System Variant")
        }

        match &new_game_state.systems[destination_system as usize] {
            data::System::Used {ships, controlled_by, ..} => {
                assert_eq!(ships, &vec![data::Ships { player: data::Color::Red, fresh: 3, damaged: 0 }, data::Ships { player: data::Color::Blue, fresh: 0, damaged: 0 }]);
                assert_eq!(controlled_by, &Some(data::Color::Red))
            }
            _ => panic!("Expected Used System Variant")
        }

    }

    #[test]
    #[should_panic(expected = "Destination not connected to Origin")]
    fn move_to_non_adjacent_system(){
        let test_setup: data::SetupCard = two_player_frontiers();
        let game_state: data::GameState = board::setup_game(&test_setup);

        let origin_system = 17;
        let destination_system = 18;

        let _ = actions::execute_actions(&game_state, vec![
            data::Action::PlayLeadCard { card: data::ActionCard { action_type: data::ActionType::Mobilization, number: 2, pips: 4, declared_ambition: Some(data::AmbitionTypes::Tycoon) }, declare: None },
            data::Action::EndPrelude,
            data::Action::Move { origin_id: origin_system, destination_id: destination_system, fresh_ships: 3, damaged_ships: 0 }
        ]);
    }

}