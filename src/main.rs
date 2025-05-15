mod data;
mod board;
mod actions;

use data::SetupCard;
use board::setup_game;

fn main() {
    let test_setup = SetupCard{players: 2,cluster_out_of_play: vec![3]};

    let inital_game_state = setup_game(&test_setup);
}