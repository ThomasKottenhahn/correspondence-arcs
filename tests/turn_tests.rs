#[cfg(test)]
mod test{
    use correspondence_arcs::data::setup_cards::{two_player_frontiers};

    use correspondence_arcs::data::game_state::{Action, ActionCard, ActionType, AmbitionTypes, BuildType, Color, GameState, TurnState};
    use correspondence_arcs::data::system::{System, BuildingSlot, BuildingType, Ships};
    
    use correspondence_arcs::board;
    use correspondence_arcs::actions;

    #[test]
    fn mobilization_then_construction(){
        let game_state = board::setup_game(&two_player_frontiers());

        let g1: GameState = actions::execute_actions(&game_state, vec![
            Action::PlayLeadCard { card: ActionCard { action_type: ActionType::Mobilization, number: 2, pips: 4, declared_ambition: Some(AmbitionTypes::Tycoon) }, declare: None },
            Action::EndPrelude,
            Action::Move { origin_id: 17, destination_id: 16, fresh_ships: 2, damaged_ships: 0 },
            Action::EndTurn
        ]);

        assert_eq!(g1.turn_state, TurnState::TrickTaking);
        assert_eq!(g1.initiative, Color::Red);
        assert_eq!(g1.current_player, Color::Blue);

        let g2: GameState = actions::execute_actions(&g1, vec![
            Action::Pivot { card: ActionCard { action_type: ActionType::Construction, number: 3, pips: 3, declared_ambition: Some(AmbitionTypes::Tyrant) }, seize: None },
            Action::EndPrelude,
            Action::Build { target_system: 18, build_type: BuildType::Ship },
            Action::EndTurn
        ]);

        assert_eq!(g2.initiative, Color::Red);
        assert_eq!(g2.current_player, Color::Red);

        let g3: GameState = actions::execute_actions(&g2, vec![
            Action::PlayLeadCard { card: ActionCard { action_type: ActionType::Construction, number: 2, pips: 4, declared_ambition: Some(AmbitionTypes::Tycoon) }, declare: None },
            Action::EndPrelude,
            Action::Build { target_system: 16, build_type: BuildType::Starport },
            Action::EndTurn
        ]);

        let g4: GameState = actions::execute_actions(&g3, vec![
            Action::Surpass { card: ActionCard { action_type: ActionType::Construction, number: 6, pips: 2, declared_ambition: Some(AmbitionTypes::Empath) }, seize: None },
            Action::EndPrelude,
            Action::Build { target_system: 18, build_type: BuildType::Ship }
        ]);

        assert_eq!(g4.initiative, Color::Blue);
        assert_eq!(g4.current_player, Color::Blue)

    }

}