#[cfg(test)]
mod test {
    use correspondence_arcs::data::setup_cards::two_player_frontiers;

    use correspondence_arcs::data::game_state::{Action, ActionCard, ActionType, AmbitionTypes, BasicAction, Color, Trophy, TrophyType};
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

        let g3 = actions::execute_actions(&g2, vec![
            Action::PlayLeadCard { card: ActionCard { action_type: ActionType::Agression, number: 2, pips: 3, declared_ambition: Some(AmbitionTypes::Tycoon) }, declare: None },
            Action::EndPrelude,
            Action::MainAction { basic_action: BasicAction::Secure { card_id: 0, vox_payload: None } }
        ]);

        assert_ne!(g3.players.get(&Color::Red).unwrap().guild_cards, vec![]);
        assert_eq!(g3.players.get(&Color::Red).unwrap().tropies, vec![Trophy{ trophy_type: TrophyType::Agent, count: 1, player: Color::Blue}])
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