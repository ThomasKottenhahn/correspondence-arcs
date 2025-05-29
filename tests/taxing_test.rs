#[cfg(test)]
mod test{
    use correspondence_arcs::data;
    use correspondence_arcs::board;
    use correspondence_arcs::actions;
    use correspondence_arcs::setup_cards::two_player_frontiers;

    #[test]
    fn test_tax_own_city(){
        let test_setup: data::SetupCard = two_player_frontiers();
        let game_state: data::GameState = board::setup_game(&test_setup);

        let target_system = 20;

        let new_game_state = actions::execute_actions(&game_state, vec![
            data::Action::PlayLeadCard { card: data::ActionCard { action_type: data::ActionType::Administration, number: 2, pips: 4, declared_ambition: Some(data::AmbitionTypes::Tycoon) }, declare: None },
            data::Action::EndPrelude,
            data::Action::Tax { target_system: target_system, target_player: data::Color::Red }
        ]);

        assert_eq!(new_game_state.turn_state, data::TurnState::AllocateResource { resource: data::ResourceType::Psionics });
        
        match new_game_state.systems[target_system as usize].clone() {
            data::System::Unused => panic!("System should not be unused"),
            data::System::Used { system_id, system_type, building_slots, ships, controlled_by, connects_to } => {
                assert_eq!(system_id, target_system);
                assert_eq!(system_type, data::SystemType::Planet { resource: data::ResourceType::Psionics });
                assert_eq!(building_slots, vec![data::BuildingSlot::Occupied { fresh: true, player: data::Color::Red, building_type: data::BuildingType::City, used: true }, data::BuildingSlot::Empty]);
                assert_eq!(ships, vec![data::Ships{player: data::Color::Red, fresh: 3, damaged: 0}, data::Ships{player: data::Color::Blue, fresh: 0, damaged: 0}]);
                assert_eq!(controlled_by, Some(data::Color::Red));
                assert_eq!(connects_to, vec![4,19]);
            },
        }

    }

}