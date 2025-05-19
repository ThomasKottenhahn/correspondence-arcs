

#[cfg(test)]
mod test{
    use correspondence_arcs::data;
    use correspondence_arcs::board;
    use correspondence_arcs::actions;

    #[test]
    fn test_building_ships(){
        let test_setup: data::SetupCard = data::SetupCard{players: 2,cluster_out_of_play: vec![3]};
        let game_state: data::GameState = board::setup_game(&test_setup);

        let target_system = 0;
        let build_type = data::BuildType::Ship;

        let new_game_state = actions::build(&game_state, target_system, build_type);

        match &new_game_state.systems[target_system as usize] {
            data::System::Used {ships, controlled_by, ..} => {
                assert_eq!(ships, &vec![data::Ships { player: data::Color::Red, fresh: 1, damaged: 0 }, data::Ships { player: data::Color::Blue, fresh: 0, damaged: 0 }]);
                assert_eq!(controlled_by, &Some(data::Color::Red));
            }
            _ => panic!("Expected Used system variant"),
        }
    }

    #[test]
    fn test_execute_building_action(){
        let test_setup: data::SetupCard = data::SetupCard{players: 2,cluster_out_of_play: vec![3]};
        let game_state: data::GameState = board::setup_game(&test_setup);

        let target_system = 8;
        let build_type = data::BuildType::Starport;

        let new_game_state = actions::execute_action(&game_state, data::Action::Build{target_system, build_type});

        match &new_game_state.systems[target_system as usize] {
            data::System::Used {building_slots, controlled_by, ..} => {
                assert_eq!(building_slots, &vec![data::BuildingSlot::Occupied { fresh: true, player: data::Color::Red, building_type: data::BuildingType::Starport}, data::BuildingSlot::Empty]);
                assert_eq!(controlled_by, &None);
            }
            _ => panic!("Expected Used system variant"),
        }
    }
}