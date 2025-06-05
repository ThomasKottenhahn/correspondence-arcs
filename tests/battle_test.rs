#[cfg(test)]
mod test {
    use correspondence_arcs::data::setup_cards::{SetupCard, two_player_frontiers};

    use correspondence_arcs::data::game_state::{GameState, TurnState, ActionCard, Action, ActionType, AmbitionTypes, Color, Dice};

    use correspondence_arcs::board;
    use correspondence_arcs::actions;

    #[test]
    fn roll_one_skirmish_dice() {
        let test_setup: SetupCard = two_player_frontiers();
        let mut game_state: GameState = board::setup_game(&test_setup);

        game_state.add_action_cards(&Color::Red, vec![ActionCard { action_type: ActionType::Agression, number: 2, pips: 3, declared_ambition: Some(AmbitionTypes::Tycoon) }]);

        let game_state = actions::move_ships(&game_state, 17, 16, 1, 0);
        let game_state = actions::move_ships(&game_state, 16, 15, 1, 0);

        let new_game_state = actions::execute_actions(&game_state, vec![
            Action::PlayLeadCard { card: ActionCard { action_type: ActionType::Agression, number: 2, pips: 3, declared_ambition: Some(AmbitionTypes::Tycoon) }, declare: None },
            Action::EndPrelude,
            Action::Battle { target_system: 15, target_player: Color::Blue, dice: vec![Dice::Skirmish] }
        ]);

        match new_game_state.turn_state {
            TurnState::AllocateDiceResults { target_system, target_player, self_hits, hits, building_hits, keys } => {
                assert_eq!(target_system, 15);
                assert_eq!(target_player, Color::Blue);
                assert_eq!(self_hits,0);
                assert_eq!(building_hits,0);
                assert_eq!(keys,0);
                match hits {
                    0..2 => {},
                    _ => panic!("Got {:?} hits when rolling 1 Skirmish Dice", hits)
                }
            }
            _ => panic!("Incorrect Turnstate")
        }
    }

    #[test]
    fn roll_three_skirmish_dice() {
        let test_setup: SetupCard = two_player_frontiers();
        let mut game_state: GameState = board::setup_game(&test_setup);

        game_state.add_action_cards(&Color::Red, vec![ActionCard { action_type: ActionType::Agression, number: 2, pips: 3, declared_ambition: Some(AmbitionTypes::Tycoon) }]);

        let game_state = actions::move_ships(&game_state, 17, 16, 3, 0);
        let game_state = actions::move_ships(&game_state, 16, 15, 3, 0);

        let new_game_state = actions::execute_actions(&game_state, vec![
            Action::PlayLeadCard { card: ActionCard { action_type: ActionType::Agression, number: 2, pips: 3, declared_ambition: Some(AmbitionTypes::Tycoon) }, declare: None },
            Action::EndPrelude,
            Action::Battle { target_system: 15, target_player: Color::Blue, dice: vec![Dice::Skirmish,Dice::Skirmish,Dice::Skirmish] }
        ]);

        match new_game_state.turn_state {
            TurnState::AllocateDiceResults { target_system, target_player, self_hits, hits, building_hits, keys } => {
                assert_eq!(target_system, 15);
                assert_eq!(target_player, Color::Blue);
                assert_eq!(self_hits,0);
                assert_eq!(building_hits,0);
                assert_eq!(keys,0);
                match hits {
                    0..4 => {},
                    _ => panic!("Got {:?} hits when rolling 3 Skirmish Dice", hits)
                }
            }
            _ => panic!("Incorrect Turnstate")
        }
    }




    #[test]
    #[should_panic(expected="Cannot roll more dice than ships present")]
    fn roll_more_dice_than_ships(){
        let test_setup: SetupCard = two_player_frontiers();
        let mut game_state: GameState = board::setup_game(&test_setup);

        game_state.add_action_cards(&Color::Red, vec![ActionCard { action_type: ActionType::Agression, number: 2, pips: 3, declared_ambition: Some(AmbitionTypes::Tycoon) }]);

        let game_state = actions::move_ships(&game_state, 17, 16, 1, 0);
        let game_state = actions::move_ships(&game_state, 16, 15, 1, 0);

        let _ = actions::execute_actions(&game_state, vec![
            Action::PlayLeadCard { card: ActionCard { action_type: ActionType::Agression, number: 2, pips: 3, declared_ambition: Some(AmbitionTypes::Tycoon) }, declare: None },
            Action::EndPrelude,
            Action::Battle { target_system: 15, target_player: Color::Blue, dice: vec![Dice::Skirmish,Dice::Skirmish] }
        ]);
    }

    #[test]
    #[should_panic(expected="Cannot battle Blue in System without presence.")]
    fn battle_without_target(){
        let test_setup: SetupCard = two_player_frontiers();
        let mut game_state: GameState = board::setup_game(&test_setup);

        game_state.add_action_cards(&Color::Red, vec![ActionCard { action_type: ActionType::Agression, number: 2, pips: 3, declared_ambition: Some(AmbitionTypes::Tycoon) }]);

        let game_state = actions::move_ships(&game_state, 17, 16, 1, 0);

        let _ = actions::execute_actions(&game_state, vec![
            Action::PlayLeadCard { card: ActionCard { action_type: ActionType::Agression, number: 2, pips: 3, declared_ambition: Some(AmbitionTypes::Tycoon) }, declare: None },
            Action::EndPrelude,
            Action::Battle { target_system: 16, target_player: Color::Blue, dice: vec![Dice::Skirmish] }
        ]);
    }

}