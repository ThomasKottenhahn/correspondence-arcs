mod data;
mod board;

use data::SetupCard;
use board::create_reach;

fn main() {
    let test_setup = SetupCard{players: 2,cluster_out_of_play: vec![3]};

    create_reach(&test_setup);
}