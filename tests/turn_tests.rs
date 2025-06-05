#[cfg(test)]
mod test{
    use correspondence_arcs::data::setup_cards::{two_player_frontiers};

    use correspondence_arcs::data::game_state::{Action, ActionCard, ActionType, Ambition, AmbitionMarker, AmbitionTypes, BuildType, Color, GameState, ResourceType, TurnState};
    use correspondence_arcs::data::system::{BuildingSlot, BuildingType, Ships, System, SystemType};
    
    use correspondence_arcs::board;
    use correspondence_arcs::actions;

    #[test]
    fn mobilization_then_construction() {
        let mut game_state = board::setup_game(&two_player_frontiers());

        game_state.add_action_cards(&Color::Red, vec![
            ActionCard { action_type: ActionType::Mobilization, number: 2, pips: 4, declared_ambition: Some(AmbitionTypes::Tycoon) },
            ActionCard { action_type: ActionType::Construction, number: 2, pips: 4, declared_ambition: Some(AmbitionTypes::Tycoon) }
            ]);

        game_state.add_action_cards(&Color::Blue, vec![
            ActionCard { action_type: ActionType::Construction, number: 6, pips: 2, declared_ambition: Some(AmbitionTypes::Empath) },
            ActionCard { action_type: ActionType::Construction, number: 3, pips: 3, declared_ambition: Some(AmbitionTypes::Tyrant) }
            ]);

        assert_eq!(game_state.current_player, Color::Red);

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

        let g4 = actions::execute_action(&g4, Action::EndTurn);

        assert_eq!(g4.initiative, Color::Blue);
        assert_eq!(g4.current_player, Color::Blue);

        match &g4.systems[17] {
            System::Unused => panic!("Expected Used System"),
            System::Used { 
                system_id, 
                system_type, 
                building_slots, 
                ships, 
                controlled_by, 
                connects_to } => {
                    assert_eq!(system_id, &17);
                    assert_eq!(system_type, &SystemType::Planet { resource: ResourceType::Material });
                    assert_eq!(building_slots, &vec![BuildingSlot::Occupied { fresh: true, player: Color::Red, building_type: BuildingType::Starport, used: false }]);
                    assert_eq!(ships, &vec![Ships { player: Color::Red, fresh: 1, damaged: 0}, Ships { player: Color::Blue, fresh: 0, damaged: 0}]);
                    assert_eq!(controlled_by, &Some(Color::Red));
                    assert_eq!(connects_to, &vec![3, 16])
                },
        }

        match &g4.systems[16] {
            System::Unused => panic!("Expected Used System"),
            System::Used { 
                system_id, 
                system_type, 
                building_slots, 
                ships, 
                controlled_by, 
                connects_to } => {
                    assert_eq!(system_id, &16);
                    assert_eq!(system_type, &SystemType::Planet { resource: ResourceType::Fuel });
                    assert_eq!(building_slots, &vec![BuildingSlot::Occupied { fresh: true, player: Color::Red, building_type: BuildingType::Starport, used: false }, BuildingSlot::Empty]);
                    assert_eq!(ships, &vec![Ships { player: Color::Red, fresh: 2, damaged: 0}, Ships { player: Color::Blue, fresh: 0, damaged: 0}]);
                    assert_eq!(controlled_by, &Some(Color::Red));
                    assert_eq!(connects_to, &vec![3, 15, 17])
                },
        }

        match &g4.systems[18] {
            System::Unused => panic!("Expected Used System"),
            System::Used { 
                system_id, 
                system_type, 
                building_slots, 
                ships, 
                controlled_by, 
                connects_to } => {
                    assert_eq!(system_id, &18);
                    assert_eq!(system_type, &SystemType::Planet { resource: ResourceType::Weapons });
                    assert_eq!(building_slots, &vec![BuildingSlot::Occupied { fresh: true, player: Color::Blue, building_type: BuildingType::Starport, used: false }]);
                    assert_eq!(ships, &vec![Ships { player: Color::Red, fresh: 0, damaged: 0}, Ships { player: Color::Blue, fresh: 5, damaged: 0}]);
                    assert_eq!(controlled_by, &Some(Color::Blue));
                    assert_eq!(connects_to, &vec![4, 19])
                },
        }

    }

    #[test]
    fn declare_ambition() {
        let mut game_state = board::setup_game(&two_player_frontiers());

        game_state.add_action_cards(&Color::Red, vec![
            ActionCard { action_type: ActionType::Mobilization, number: 2, pips: 4, declared_ambition: Some(AmbitionTypes::Tycoon) },
            ActionCard { action_type: ActionType::Construction, number: 2, pips: 4, declared_ambition: Some(AmbitionTypes::Tycoon) }
            ]);

        game_state.add_action_cards(&Color::Blue, vec![
            ActionCard { action_type: ActionType::Construction, number: 6, pips: 2, declared_ambition: Some(AmbitionTypes::Empath) },
            ActionCard { action_type: ActionType::Construction, number: 3, pips: 3, declared_ambition: Some(AmbitionTypes::Tyrant) }
            ]);

        assert_eq!(game_state.current_player, Color::Red);

        let new_game_state = actions::execute_action(&game_state, Action::PlayLeadCard { card: ActionCard { action_type: ActionType::Mobilization, number: 2, pips: 4, declared_ambition: Some(AmbitionTypes::Tycoon) }, declare: Some(AmbitionTypes::Tycoon) });

        assert_eq!(new_game_state.ambition_markers, vec![AmbitionMarker {first_place: 2, second_place: 0, flipped: false, first_place_flipped: 4, second_place_flipped: 2},AmbitionMarker {first_place: 3, second_place: 2, flipped: false, first_place_flipped: 6, second_place_flipped: 4}]);
        assert_eq!(new_game_state.ambitions.get(&AmbitionTypes::Tycoon), Some(&Ambition{ambition_type: AmbitionTypes::Tycoon, markers: vec![AmbitionMarker {first_place: 5, second_place: 3, flipped: false, first_place_flipped: 9, second_place_flipped: 5}], discarded_resources: vec![]}))

    }

}