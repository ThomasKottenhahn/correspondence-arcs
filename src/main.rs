mod data;
mod board;
mod actions;
mod setup_cards;

use data::SetupCard;
use board::setup_game;

fn main() {
    let test_setup: SetupCard = setup_cards::two_player_frontiers();

    let inital_game_state = setup_game(&test_setup);

    print!("{:?}", inital_game_state.systems.iter().map(|x|{
        match x {
            data::System::Unused => "Unused".to_string(),
            data::System::Used {controlled_by: Some(_), ..} => "Controlled".to_string(),
            data::System::Used {controlled_by: None, ..} => "None".to_string()
        }
    }).collect::<Vec<String>>())
}