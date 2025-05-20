mod data;
mod board;
mod actions;

use data::SetupCard;
use board::setup_game;

fn main() {
    let test_setup: data::SetupCard = data::SetupCard { players: 2, cluster_out_of_play: vec![0,5], a_locations: vec![19,11], b_locations: vec![16, 17], c_loactions: vec![2,13,4,14]};

    let inital_game_state = setup_game(&test_setup);
}