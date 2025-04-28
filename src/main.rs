mod data;

fn main() {
    let test_setup = data::SetupCard{players: 2,cluster_out_of_play: vec![3]};

    data::create_reach(&test_setup);
}