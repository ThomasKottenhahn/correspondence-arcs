#[cfg(test)]
mod test {
    use correspondence_arcs::data::setup_cards::{SetupCard, two_player_frontiers};

    use correspondence_arcs::data::game_state::{GameState, ActionCard, Action, ActionType, AmbitionTypes, Color};
    use correspondence_arcs::data::system::{Ships, System};
    use correspondence_arcs::board;
    use correspondence_arcs::actions;

    #[test]
    fn move_1_ship() {
        let test_setup: SetupCard = two_player_frontiers();
        let game_state: GameState = board::setup_game(&test_setup);

        let origin_system = 17;
        let destination_system = 3;

        let new_game_state = actions::execute_actions(&game_state, vec![
            Action::PlayLeadCard { card: ActionCard { action_type: ActionType::Mobilization, number: 2, pips: 4, declared_ambition: Some(AmbitionTypes::Tycoon) }, declare: None },
            Action::EndPrelude,
            Action::Move { origin_id: origin_system, destination_id: destination_system, fresh_ships: 1, damaged_ships: 0 }
        ]);

        match &new_game_state.systems[origin_system as usize] {
            System::Used {ships, controlled_by, ..} => {
                assert_eq!(ships, &vec![Ships { player: Color::Red, fresh: 2, damaged: 0 }, Ships { player: Color::Blue, fresh: 0, damaged: 0 }]);
                assert_eq!(controlled_by, &Some(Color::Red))
            }
            _ => panic!("Expected Used System Variant")
        }

        match &new_game_state.systems[destination_system as usize] {
            System::Used {ships, controlled_by, ..} => {
                assert_eq!(ships, &vec![Ships { player: Color::Red, fresh: 1, damaged: 0 }, Ships { player: Color::Blue, fresh: 0, damaged: 0 }]);
                assert_eq!(controlled_by, &Some(Color::Red))
            }
            _ => panic!("Expected Used System Variant")
        }

    }

    #[test]
    fn move_all_ships() {
        let test_setup: SetupCard = two_player_frontiers();
        let game_state: GameState = board::setup_game(&test_setup);

        let origin_system = 17;
        let destination_system = 16;

        let new_game_state = actions::execute_actions(&game_state, vec![
            Action::PlayLeadCard { card: ActionCard { action_type: ActionType::Mobilization, number: 2, pips: 4, declared_ambition: Some(AmbitionTypes::Tycoon) }, declare: None },
            Action::EndPrelude,
            Action::Move { origin_id: origin_system, destination_id: destination_system, fresh_ships: 3, damaged_ships: 0 }
        ]);

        match &new_game_state.systems[origin_system as usize] {
            System::Used {ships, controlled_by, ..} => {
                assert_eq!(ships, &vec![Ships { player: Color::Red, fresh: 0, damaged: 0 }, Ships { player: Color::Blue, fresh: 0, damaged: 0 }]);
                assert_eq!(controlled_by, &None)
            }
            _ => panic!("Expected Used System Variant")
        }

        match &new_game_state.systems[destination_system as usize] {
            System::Used {ships, controlled_by, ..} => {
                assert_eq!(ships, &vec![Ships { player: Color::Red, fresh: 3, damaged: 0 }, Ships { player: Color::Blue, fresh: 0, damaged: 0 }]);
                assert_eq!(controlled_by, &Some(Color::Red))
            }
            _ => panic!("Expected Used System Variant")
        }

    }

    #[test]
    #[should_panic(expected = "Destination not connected to Origin")]
    fn move_to_non_adjacent_system(){
        let test_setup: SetupCard = two_player_frontiers();
        let game_state: GameState = board::setup_game(&test_setup);

        let origin_system = 17;
        let destination_system = 18;

        let _ = actions::execute_actions(&game_state, vec![
            Action::PlayLeadCard { card: ActionCard { action_type: ActionType::Mobilization, number: 2, pips: 4, declared_ambition: Some(AmbitionTypes::Tycoon) }, declare: None },
            Action::EndPrelude,
            Action::Move { origin_id: origin_system, destination_id: destination_system, fresh_ships: 3, damaged_ships: 0 }
        ]);
    }

}