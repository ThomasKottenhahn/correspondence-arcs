

#[cfg(test)]
mod test{
    use correspondence_arcs::data;
    use correspondence_arcs::board;
    use correspondence_arcs::actions;

    #[test]
    fn test_building(){
        let test_setup: data::SetupCard = data::SetupCard{players: 2,cluster_out_of_play: vec![3]};
        let game_state: data::GameState = board::setup_game(&test_setup);

        let target_system = 0;
        let build_type = data::BuildType::Ship;

        let new_game_state = actions::build(&game_state, target_system, build_type);
        let current_player = new_game_state.current_player.clone();

        match &new_game_state.systems[target_system as usize] {
            data::System::Used {ships, ..} => {
                assert_eq!(ships, &vec![data::Ships { player: current_player, fresh: 1, damaged: 0 }])
            }
            _ => panic!("Expected Used system variant"),
        }
    }
}