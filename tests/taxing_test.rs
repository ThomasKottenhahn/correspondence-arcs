#[cfg(test)]
mod test{
    use correspondence_arcs::data::setup_cards::{SetupCard, two_player_frontiers};

    use correspondence_arcs::data::game_state::{GameState, TurnState, ResourceType, ActionCard, Action, ActionType, AmbitionTypes, Color};
    use correspondence_arcs::data::system::{Ships, System, SystemType, BuildingSlot, BuildingType};
    use correspondence_arcs::board;
    use correspondence_arcs::actions;

    #[test]
    fn test_tax_own_city(){
        let test_setup: SetupCard = two_player_frontiers();
        let game_state: GameState = board::setup_game(&test_setup);

        let target_system = 20;

        let new_game_state = actions::execute_actions(&game_state, vec![
            Action::PlayLeadCard { card: ActionCard { action_type: ActionType::Administration, number: 2, pips: 4, declared_ambition: Some(AmbitionTypes::Tycoon) }, declare: None },
            Action::EndPrelude,
            Action::Tax { target_system: target_system, target_player: Color::Red }
        ]);

        assert_eq!(new_game_state.turn_state, TurnState::AllocateResource { resource: ResourceType::Psionics });
        
        match new_game_state.systems[target_system as usize].clone() {
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

    }

}