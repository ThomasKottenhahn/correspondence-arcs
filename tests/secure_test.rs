#[cfg(test)]
mod test {
    use correspondence_arcs::data::court_cards::VoxPayload;
    use correspondence_arcs::data::setup_cards::two_player_frontiers;

    use correspondence_arcs::data::game_state::{Action, ActionCard, ActionType, AmbitionTypes, BasicAction, Color, ReserveType, Trophy};
    use correspondence_arcs::board::setup_game_with_set_seed;
    use correspondence_arcs::actions;

    #[test]
    fn secure_guild_card() {
        let mut game_state = setup_game_with_set_seed(&two_player_frontiers(),11);

        game_state.add_action_cards(&Color::Red, vec![
            ActionCard { action_type: ActionType::Administration, number: 2, pips: 4, declared_ambition: Some(AmbitionTypes::Tycoon) },
            ActionCard { action_type: ActionType::Agression, number: 2, pips: 3, declared_ambition: Some(AmbitionTypes::Tycoon) }
            ]);

        game_state.add_action_cards(&Color::Blue, vec![
            ActionCard { action_type: ActionType::Mobilization, number: 6, pips: 2, declared_ambition: Some(AmbitionTypes::Empath) }
            ]);

        let g1 = actions::execute_actions(&game_state, vec![
            Action::PlayLeadCard { card: ActionCard { action_type: ActionType::Administration, number: 2, pips: 4, declared_ambition: Some(AmbitionTypes::Tycoon) }, declare: None },
            Action::EndPrelude,
            Action::MainAction { basic_action: BasicAction::Influence { card_id: 0 } },
            Action::MainAction { basic_action: BasicAction::Influence { card_id: 0 } },
            Action::EndTurn
        ]);

        let g2 = actions::execute_actions(&g1, vec![
            Action::Pivot { card: ActionCard { action_type: ActionType::Mobilization, number: 6, pips: 2, declared_ambition: Some(AmbitionTypes::Empath) }, seize: None },
            Action::EndPrelude,
            Action::MainAction { basic_action: BasicAction::Influence { card_id: 0 } },
            Action::EndTurn
        ]);

        assert_eq!(g2.players.get(&Color::Red).unwrap().reserve.get(&ReserveType::Agents).unwrap(), &8);
        assert_eq!(g2.players.get(&Color::Blue).unwrap().reserve.get(&ReserveType::Agents).unwrap(), &9);

        let g3 = actions::execute_actions(&g2, vec![
            Action::PlayLeadCard { card: ActionCard { action_type: ActionType::Agression, number: 2, pips: 3, declared_ambition: Some(AmbitionTypes::Tycoon) }, declare: None },
            Action::EndPrelude,
            Action::MainAction { basic_action: BasicAction::Secure { card_id: 0, vox_payload: None } }
        ]);

        assert_eq!(g3.players.get(&Color::Red).unwrap().reserve.get(&ReserveType::Agents).unwrap(), &10);
        assert_eq!(g3.players.get(&Color::Blue).unwrap().reserve.get(&ReserveType::Agents).unwrap(), &9);

        assert_ne!(g3.players.get(&Color::Red).unwrap().guild_cards, vec![]);
        assert_eq!(g3.players.get(&Color::Red).unwrap().tropies, vec![Trophy{ trophy_type: ReserveType::Agents, count: 1, player: Color::Blue}])
    }

    #[test]
    fn secure_mass_uprising() {
        let mut game_state = setup_game_with_set_seed(&two_player_frontiers(),11);

        game_state.add_action_cards(&Color::Red, vec![
            ActionCard { action_type: ActionType::Administration, number: 2, pips: 4, declared_ambition: Some(AmbitionTypes::Tycoon) },
            ActionCard { action_type: ActionType::Agression, number: 2, pips: 3, declared_ambition: Some(AmbitionTypes::Tycoon) }
            ]);

        game_state.add_action_cards(&Color::Blue, vec![
            ActionCard { action_type: ActionType::Mobilization, number: 6, pips: 2, declared_ambition: Some(AmbitionTypes::Empath) }
            ]);

        let g1 = actions::execute_actions(&game_state, vec![
            Action::PlayLeadCard { card: ActionCard { action_type: ActionType::Administration, number: 2, pips: 4, declared_ambition: Some(AmbitionTypes::Tycoon) }, declare: None },
            Action::EndPrelude,
            Action::MainAction { basic_action: BasicAction::Influence { card_id: 1 } },
            Action::MainAction { basic_action: BasicAction::Influence { card_id: 1 } },
            Action::EndTurn
        ]);

        let g2 = actions::execute_actions(&g1, vec![
            Action::Pivot { card: ActionCard { action_type: ActionType::Mobilization, number: 6, pips: 2, declared_ambition: Some(AmbitionTypes::Empath) }, seize: None },
            Action::EndPrelude,
            Action::MainAction { basic_action: BasicAction::Influence { card_id: 1 } },
            Action::EndTurn
        ]);

        assert_eq!(g2.players.get(&Color::Red).unwrap().reserve.get(&ReserveType::Agents).unwrap(), &8);
        assert_eq!(g2.players.get(&Color::Blue).unwrap().reserve.get(&ReserveType::Agents).unwrap(), &9);

        let g3 = actions::execute_actions(&g2, vec![
            Action::PlayLeadCard { card: ActionCard { action_type: ActionType::Agression, number: 2, pips: 3, declared_ambition: Some(AmbitionTypes::Tycoon) }, declare: None },
            Action::EndPrelude,
            Action::MainAction { basic_action: BasicAction::Secure { card_id: 1, vox_payload: Some(VoxPayload::MassUprising { target_systems: vec![2,12,13,14] })  } }
        ]);

        assert_eq!(g3.players.get(&Color::Red).unwrap().reserve.get(&ReserveType::Agents).unwrap(), &10);
        assert_eq!(g3.players.get(&Color::Blue).unwrap().reserve.get(&ReserveType::Agents).unwrap(), &9);

        assert_eq!(g3.players.get(&Color::Red).unwrap().tropies, vec![Trophy{ trophy_type: ReserveType::Agents, count: 1, player: Color::Blue}]);
        assert_eq!(g3.systems[2].get_all_ships(&Color::Red), 3);
        assert_eq!(g3.systems[12].get_all_ships(&Color::Red), 1);
        assert_eq!(g3.systems[12].get_all_ships(&Color::Blue), 3);
        assert_eq!(g3.systems[13].get_all_ships(&Color::Red), 1);
        assert_eq!(g3.systems[14].get_all_ships(&Color::Red), 3);
    }

    #[test]
    #[should_panic(expected = "Can only secure controlled Card")]
    fn secure_not_controlled_guild_card() {
        let mut game_state = setup_game_with_set_seed(&two_player_frontiers(),11);

        game_state.add_action_cards(&Color::Red, vec![
            ActionCard { action_type: ActionType::Agression, number: 2, pips: 3, declared_ambition: Some(AmbitionTypes::Tycoon) }
            ]);

        let _ = actions::execute_actions(&game_state, vec![
            Action::PlayLeadCard { card: ActionCard { action_type: ActionType::Agression, number: 2, pips: 3, declared_ambition: Some(AmbitionTypes::Tycoon) }, declare: None },
            Action::EndPrelude,
            Action::MainAction { basic_action: BasicAction::Secure { card_id: 0, vox_payload: None } }
        ]);
    }
}