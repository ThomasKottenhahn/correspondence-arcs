use crate::data::SetupCard;

pub fn two_player_frontiers() -> SetupCard {return SetupCard { players: 2, cluster_out_of_play: vec![0,5], a_locations: vec![20,12], b_locations: vec![17, 18], c_locations: vec![2,4,14,15]}}