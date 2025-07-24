#[cfg(test)]
mod test{
    use correspondence_arcs::data::setup_cards::{SetupCard, two_player_frontiers};

    use correspondence_arcs::data::game_state::{Action, ActionCard, ActionType, AmbitionTypes, BasicAction, Color, GameState, ReserveType, ResourceType, TurnState, ResourceSlot};
    use correspondence_arcs::data::system::{Ships, System, SystemType, BuildingSlot, BuildingType};
    use correspondence_arcs::board;
    use correspondence_arcs::actions;

    #[test]
    fn test_tax_own_city(){
        let test_setup: SetupCard = two_player_frontiers();
        let mut game_state: GameState = board::setup_game(&test_setup);

        game_state.add_action_cards(&Color::Red, vec![ActionCard { action_type: ActionType::Administration, number: 2, pips: 4, declared_ambition: Some(AmbitionTypes::Tycoon) }]);

        let target_system = 20;

        let g1 = actions::execute_actions(&game_state, vec![
            Action::PlayLeadCard { card: ActionCard { action_type: ActionType::Administration, number: 2, pips: 4, declared_ambition: Some(AmbitionTypes::Tycoon) }, declare: None },
            Action::EndPrelude,
            Action::MainAction {basic_action: BasicAction::Tax { target_system: target_system, target_player: Color::Red }}
        ]);

        assert_eq!(g1.turn_state, TurnState::AllocateResources { player: Color::Red, resources: vec![ResourceType::Psionics] });
        
        match g1.systems[target_system as usize].clone() {
            System::Unused => panic!("System should not be unused"),
            System::Used { system_id, system_type, building_slots, ships, controlled_by, connects_to } => {
                assert_eq!(system_id, target_system);
                assert_eq!(system_type, SystemType::Planet { resource: ResourceType::Psionics });
                assert_eq!(building_slots, vec![BuildingSlot::Occupied { fresh: true, player: Color::Red, building_type: BuildingType::City, used: true }, BuildingSlot::Empty]);
                assert_eq!(ships, vec![Ships{player: Color::Red, fresh: 3, damaged: 0}, Ships{player: Color::Blue, fresh: 0, damaged: 0}]);
                assert_eq!(controlled_by, Some(Color::Red));
                assert_eq!(connects_to, vec![4,19]);
            },
        }

        let g2 = actions::execute_action(&g1, Action::AllocateResources { configuration: vec![(0,ResourceType::Psionics), (1,ResourceType::Material), (2,ResourceType::Psionics)] });

        assert_eq!(
            g2.players.get(&Color::Red).unwrap().resource_slots,
            vec![ResourceSlot::Used { keys: 3, resource: ResourceType::Psionics }, ResourceSlot::Used { keys: 1, resource: ResourceType::Material }, ResourceSlot::Used { keys: 1, resource: ResourceType::Psionics }, ResourceSlot::Covered { keys: 2 }, ResourceSlot::Covered { keys: 1 }, ResourceSlot::Covered { keys: 3 }]
        );

        assert_eq!(g2.turn_state, TurnState::Actions { action_type: ActionType::Administration, pips_left: 3 });
        assert_eq!(g2.next_turn_states, vec![]);

    }

    #[test]
    fn test_tax_rival_city() {
        let test_setup: SetupCard = two_player_frontiers();
        let mut game_state: GameState = board::setup_game(&test_setup);
        
        game_state.add_action_cards(&Color::Red, vec![
            ActionCard { action_type: ActionType::Administration, number: 2, pips: 4, declared_ambition: Some(AmbitionTypes::Tycoon) },
            ActionCard { action_type: ActionType::Mobilization, number: 2, pips: 4, declared_ambition: Some(AmbitionTypes::Tycoon) }
            ]);
        game_state.add_action_cards(&Color::Blue, vec![
            ActionCard { action_type: ActionType::Mobilization, number: 6, pips: 2, declared_ambition: Some(AmbitionTypes::Empath) }
        ]);

        let target_system = 12;

        let g1 = actions::execute_actions(&game_state, vec![
            Action::PlayLeadCard { card: ActionCard { action_type: ActionType::Mobilization, number: 2, pips: 4, declared_ambition: Some(AmbitionTypes::Tycoon) }, declare: None },
            Action::EndPrelude,
            Action::MainAction {basic_action: BasicAction::Move { origin_id: 17, destination_id: 3, fresh_ships: 3, damaged_ships: 0 }},
            Action::MainAction {basic_action: BasicAction::Move { origin_id: 3, destination_id: 2, fresh_ships: 3, damaged_ships: 0 }},
            Action::MainAction {basic_action: BasicAction::Move { origin_id: 2, destination_id: 12, fresh_ships: 3, damaged_ships: 0 }}
        ]);

        assert_eq!(g1.systems[target_system as usize], System::Used {
            system_id: target_system,
            system_type: SystemType::Planet { resource: ResourceType::Material },
            building_slots: vec![BuildingSlot::Occupied { fresh: true, player: Color::Blue, building_type: BuildingType::City, used: false }],
            ships: vec![Ships{player: Color::Red, fresh: 3, damaged: 0}, Ships{player: Color::Blue, fresh: 3, damaged: 0}],
            controlled_by: None,
            connects_to: vec![2, 13, 11]
        });

        let g2 = actions::execute_actions(&g1, vec![
            Action::EndTurn,
            Action::Copy { card: ActionCard { action_type: ActionType::Mobilization, number: 6, pips: 2, declared_ambition: Some(AmbitionTypes::Empath) }, seize: None },
            Action::EndPrelude,
            Action::MainAction {basic_action: BasicAction::Move { origin_id: 12, destination_id: 13, fresh_ships: 1, damaged_ships: 0 }},
            Action::EndTurn,
        ]);

        assert_eq!(g2.systems[target_system as usize], System::Used {
            system_id: target_system,
            system_type: SystemType::Planet { resource: ResourceType::Material },
            building_slots: vec![BuildingSlot::Occupied { fresh: true, player: Color::Blue, building_type: BuildingType::City, used: false }],
            ships: vec![Ships{player: Color::Red, fresh: 3, damaged: 0}, Ships{player: Color::Blue, fresh: 2, damaged: 0}],
            controlled_by: Some(Color::Red),
            connects_to: vec![2, 13, 11]
        });

        assert_eq!(g2.turn_state, TurnState::TrickTaking);
        assert_eq!(g2.current_player, Color::Red);
        assert_eq!(g2.initiative, Color::Red);

        let g3 = actions::execute_actions(&g2, vec![
            Action::PlayLeadCard { card: ActionCard { action_type: ActionType::Administration, number: 2, pips: 4, declared_ambition: Some(AmbitionTypes::Tycoon) }, declare: None },
            Action::EndPrelude,
            Action::MainAction {basic_action: BasicAction::Tax { target_system: target_system, target_player: Color::Blue }}
            ]);

        assert_eq!(g3.turn_state, TurnState::AllocateResources { player: Color::Red, resources: vec![ResourceType::Material] });
        assert_eq!(g3.players.get(&Color::Blue).unwrap().reserve.get(&ReserveType::Agents), Some(&9));

    }

}