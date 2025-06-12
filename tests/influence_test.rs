#[cfg(test)]
mod test{
    use correspondence_arcs::data::setup_cards::{two_player_frontiers};

    use correspondence_arcs::data::game_state::{Action, ActionCard, ActionType, Ambition, AmbitionMarker, AmbitionTypes, BuildType, Color, GameState, ResourceType, TurnState};
    use correspondence_arcs::data::system::{BuildingSlot, BuildingType, Ships, System, SystemType};
    
    use correspondence_arcs::board;
    use correspondence_arcs::actions::{execute_action, execute_actions};

    #[test]
    fn influence_once_then_twice() {
        let mut game_state = board::setup_game(&two_player_frontiers());

        game_state.add_action_cards(&Color::Red, vec![
            ActionCard { action_type: ActionType::Administration, number: 2, pips: 4, declared_ambition: Some(AmbitionTypes::Tycoon) }
            ]);

        game_state.add_action_cards(&Color::Blue, vec![
            ActionCard { action_type: ActionType::Administration, number: 6, pips: 2, declared_ambition: Some(AmbitionTypes::Empath) }
            ]);

        print!("{:?}", game_state.court[0]);
        assert_eq!(game_state.court[0].controlled_by(), None);
        

        let g1 = execute_actions(&game_state, vec![
            Action::PlayLeadCard { card: ActionCard { action_type: ActionType::Administration, number: 2, pips: 4, declared_ambition: Some(AmbitionTypes::Tycoon) }, declare: None },
            Action::EndPrelude,
            Action::Influence { card_id: 0 },
            Action::EndTurn
        ]);

        print!("{:?}", game_state.lead_card);
        assert_eq!(g1.court[0].controlled_by(), Some(Color::Red));

        let g2 = execute_actions(&g1, vec![
            Action::Surpass { card: ActionCard { action_type: ActionType::Administration, number: 6, pips: 2, declared_ambition: Some(AmbitionTypes::Empath) }, seize: None },
            Action::EndPrelude,
            Action::Influence { card_id: 0 }
        ]); 

        assert_eq!(g2.court[0].controlled_by(), None);

        let g3 = execute_action(&g2, Action::Influence { card_id: 0 });

        assert_eq!(g3.court[0].controlled_by(), Some(Color::Blue));

        }
    }