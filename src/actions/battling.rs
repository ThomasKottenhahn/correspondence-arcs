use rand::Rng;

use crate::data::game_state::{GameState, Color, Dice, TurnState};

pub(crate) fn battle(game_state: &GameState, target_system: u8, target_player: Color, dice: Vec<Dice>) -> GameState {
    let mut rng = rand::thread_rng();

    let battle_system = &game_state.systems[target_system as usize];
    let current_player = &game_state.current_player;

    if dice.len() > battle_system.get_all_ships(current_player) as usize {panic!("Cannot roll more dice than ships present")};
    if !battle_system.has_presence(&target_player) {panic!("Cannot battle {:?} in System without presence.", target_player)}

    let (self_hits, intecept, hits, building_hits, keys) = dice.iter().map(|d| {
        let faces: Vec<(u8, u8, u8, u8, u8)> = match d {
            Dice::Skirmish => vec![(0,0,1,0,0), (0,0,1,0,0), (0,0,1,0,0), (0,0,0,0,0), (0,0,0,0,0), (0,0,0,0,0)],
            Dice::Assault => vec![(0,0,2,0,0), (1,0,2,0,0), (0,1,1,0,0), (1,0,1,0,0), (1,0,1,0,0), (0,0,0,0,0)],
            Dice::Raid => vec![(0,1,0,0,2), (1,0,0,0,1), (0,0,0,1,1), (1,0,0,1,0), (1,0,0,1,0), (0,1,0,0,0)],
        };
        let roll = rng.gen_range(0..faces.len());
        faces[roll]
    }).fold((0, 0, 0, 0, 0), |acc, x| (
        acc.0 + x.0,
        acc.1 + x.1,
        acc.2 + x.2,
        acc.3 + x.3,
        acc.4 + x.4
    ));

    let self_hits = if intecept >= 1 {self_hits + game_state.systems[target_system as usize].get_fresh_ships(&target_player)} else {self_hits};
    let mut new_game_state = game_state.clone();
    new_game_state.next_turn_states = vec![new_game_state.turn_state];
    new_game_state.turn_state = TurnState::AllocateDiceResults { target_system: target_system, target_player: target_player, self_hits: self_hits, hits: hits, building_hits: building_hits, keys: keys };
    return new_game_state;
}
