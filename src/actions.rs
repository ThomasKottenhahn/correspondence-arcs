use core::panic;
use std::collections::HashMap;

use itertools::Itertools;
use rand::Rng;

use crate::data::game_state::{Action, ActionCard, ActionType, Agents, Ambition, AmbitionTypes, BasicAction, BuildType, Color, Dice, GameState, PlayerArea, ReserveType, ResourceSlot, ResourceType, Trophy, TrophyType, TurnState};
use crate::data::court_cards::{CourtCard, Guild, VoxPayload};
use crate::data::system::{Ships, System, BuildingSlot, BuildingType, SystemType};

fn use_action_pip(game_state: &GameState) -> GameState {
    match &game_state.turn_state {
        TurnState::Actions { action_type, pips_left } => {
            GameState {
                turn_state: TurnState::Actions { action_type: action_type.clone(), pips_left: pips_left - 1 },
                ..game_state.clone()}
            },
            _ => panic!("Cannot use action pip in {:?}", game_state.turn_state)
    }
}

pub fn place_building(building_slots: &Vec<BuildingSlot>, building: BuildingSlot) -> Vec<BuildingSlot> {
    if building_slots.len() == 0{
        panic!("No building slots available");
    }

    if building_slots.iter().all(|x| matches!(x, BuildingSlot::Occupied { .. })) {
        panic!("All building slots are occupied");
    }

    for i in 0..building_slots.len(){
        if building_slots[i] == BuildingSlot::Empty{
            let mut building_slots = building_slots.clone();
            building_slots[i] = building;
            return building_slots;
        }
    }
    return building_slots.clone();
}

pub fn place_ships(ships: &Vec<Ships>, player: &Color, fresh: u8, damaged: u8) -> Vec<Ships> {
    let mut new_ships: Vec<Ships> = vec![];
    for ships_struct in ships{
        if &ships_struct.player == player {
            let mut ships_struct = ships_struct.clone();
            ships_struct.fresh = ships_struct.fresh + fresh;
            ships_struct.damaged = ships_struct.damaged + damaged;
            new_ships.push(ships_struct);
        } else {
            new_ships.push(ships_struct.clone());
        }
    }
    return new_ships;
}

fn remove_ships(ships: &Vec<Ships>, player: &Color, fresh: u8, damaged: u8) -> Vec<Ships> {
    let mut new_ships: Vec<Ships> = vec![];
    for ships_struct in ships{
        if &ships_struct.player == player {
            let mut ships_struct = ships_struct.clone();
            ships_struct.fresh = ships_struct.fresh.checked_sub(fresh).expect("Tried to move more fresh ships than available");
            ships_struct.damaged = ships_struct.damaged.checked_sub(damaged).expect("Tried to move more damaged ships than available");
            new_ships.push(ships_struct);
        } else {
            new_ships.push(ships_struct.clone());
        }
    }
    return new_ships;
}

fn build(game_state: &GameState, target_system: u8, build_type: BuildType) -> GameState {
    let mut game_state = game_state.clone();

    let current_player = game_state.current_player.clone();
    let system: System = game_state.systems[target_system as usize].clone();
    
    if !system.has_presence(&current_player) {panic!("Cannot build in a System without presence")}
    
    match system{
        System::Unused => panic!("System is unused"),
        System::Used {
            system_id,
            system_type,
            building_slots,
            ships,
            controlled_by,
            connects_to
        } => {
            let build_fresh = match controlled_by.clone() {
                None => true,
                Some(c) => c==current_player
            };    
        
            match build_type {
                BuildType::Ship => game_state.systems[target_system as usize] = System::Used {
                    system_id: system_id, 
                    system_type: system_type.clone(), 
                    building_slots: building_slots.clone(),
                    ships: place_ships(&ships, &current_player, if build_fresh {1} else {0}, if !build_fresh {0} else {0}), 
                    controlled_by: controlled_by.clone(), 
                    connects_to: connects_to.clone() }.use_building(&BuildingType::Starport, &current_player).update_control(),
                BuildType::City => {
                    let building = BuildingSlot::Occupied {fresh: build_fresh, player: current_player, building_type: BuildingType::City, used: false};
                    game_state.systems[target_system as usize] = System::Used {
                    system_id: system_id, 
                    system_type: system_type.clone(), 
                    building_slots: place_building(&building_slots, building), 
                    ships: ships.clone(), 
                    controlled_by: controlled_by.clone(), 
                    connects_to: connects_to.clone() }
                },   
                BuildType::Starport => {
                    let building = BuildingSlot::Occupied {fresh: build_fresh, player: current_player, building_type: BuildingType::Starport, used: false};
                    game_state.systems[target_system as usize] = System::Used {
                    system_id: system_id, 
                    system_type: system_type.clone(), 
                    building_slots: place_building(&building_slots, building), 
                    ships: ships.clone(), 
                    controlled_by: controlled_by.clone(), 
                    connects_to: connects_to.clone() }
                }
            }
            return game_state;
        }
    }
    
}

pub fn move_ships(game_state: &GameState, origin_system_id: u8, destination_system_id: u8, fresh: u8, damaged: u8) -> GameState {
    let mut game_state = game_state.clone();

    let origin_system = game_state.systems[origin_system_id as usize].clone();
    let destination_system = game_state.systems[destination_system_id as usize].clone();

    match origin_system{
        System::Unused => panic!("Origin system is unused"),
        System::Used {
            system_id,
            system_type,
            building_slots,
            ships,
            controlled_by,
            connects_to
        } => {
            if !connects_to.contains(&destination_system_id) {panic!("Destination not connected to Origin")}
            let updated_ships = remove_ships(&ships, &game_state.current_player, fresh, damaged);
            game_state.systems[origin_system_id as usize] = System::Used {
                system_id,
                system_type: system_type.clone(),
                building_slots: building_slots.clone(),
                ships: updated_ships,
                controlled_by: controlled_by.clone(),
                connects_to: connects_to.clone(),
            }.update_control();
        }
    }

    match destination_system{
        System::Unused => panic!("Destination system is unused"),
        System::Used {
            system_id,
            system_type,
            building_slots,
            ships,
            controlled_by,
            connects_to
        } => {
            let updated_ships = place_ships(&ships, &game_state.current_player, fresh, damaged);
            game_state.systems[destination_system_id as usize] = System::Used {
                system_id: system_id, 
                system_type: system_type.clone(), 
                building_slots: building_slots.clone(), 
                ships: updated_ships, 
                controlled_by: controlled_by.clone(), 
                connects_to: connects_to.clone() }.update_control()
        }
    }

    return game_state;
}

fn repair(game_state: &GameState, target_system: u8, build_type: BuildType) -> GameState {
    let mut game_state = game_state.clone();
    let system = game_state.systems[target_system as usize].clone();
    let current_player = game_state.current_player.clone();
    match system {
        System::Unused => panic!("Cannot repair in unused System"),
        System::Used {
            system_id,
            system_type,
            mut building_slots,
            ships,
            controlled_by,
            connects_to
        } => match build_type {
            BuildType::Ship => {
                let ships = remove_ships(&ships, &current_player, 0, 1);
                let ships = place_ships(&ships, &current_player, 1, 0);
                game_state.systems[target_system as usize] = System::Used {
                    system_id: system_id,
                    system_type: system_type,
                    building_slots: building_slots,
                    ships: ships,
                    controlled_by: controlled_by,
                    connects_to: connects_to
                }.update_control();
            },
            BuildType::City => {
                match (building_slots[0].clone(), building_slots[1].clone()) {
                    (BuildingSlot::Occupied { fresh: false, player: current_player, building_type: BuildingType::City, used }, _) => building_slots[0] = BuildingSlot::Occupied { fresh: true, player: current_player, building_type: BuildingType::City, used },
                    (_, BuildingSlot::Occupied { fresh: false, player: current_player, building_type: BuildingType::City, used }) => building_slots[1] = BuildingSlot::Occupied { fresh: true, player: current_player, building_type: BuildingType::City, used },
                    _ => panic!("Cannot repair, because no valid repair target exists")
                }
            },
            BuildType::Starport => {
                match (building_slots[0].clone(), building_slots[1].clone()) {
                    (BuildingSlot::Occupied { fresh: false, player: current_player, building_type: BuildingType::Starport, used }, _) => building_slots[0] = BuildingSlot::Occupied { fresh: true, player: current_player, building_type: BuildingType::Starport, used },
                    (_, BuildingSlot::Occupied { fresh: false, player: current_player, building_type: BuildingType::Starport, used }) => building_slots[1] = BuildingSlot::Occupied { fresh: true, player: current_player, building_type: BuildingType::Starport, used },
                    _ => panic!("Cannot repair, because no valid repair target exists")
                }
            }
        }
    }
    return game_state;
}

fn influence(game_state: &GameState, target_card: u8) -> GameState {
    let current_player = game_state.current_player.clone();
    let mut new_game_state = game_state.clone();

    new_game_state.court[target_card as usize] = match new_game_state.court[target_card as usize].clone() {
        CourtCard::VoxCard { vox, agents } => CourtCard::VoxCard {vox: vox, agents: agents.iter().map(|a| {
            if a.color == current_player {Agents{color: current_player.clone(), count: a.count + 1}}
            else {a.clone()}
        }).collect()},
        CourtCard::GuildCard { guild, agents } => CourtCard::GuildCard {guild: guild, agents: agents.iter().map(|a| {
            if a.color == current_player {Agents{color: current_player.clone(), count: a.count + 1}}
            else {a.clone()}
        }).collect()},
    };
    return new_game_state.update_players_reserve(&current_player, &ReserveType::Agents, -1);
}

fn battle(game_state: &GameState, target_system: u8, target_player: Color, dice: Vec<Dice>) -> GameState {
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

fn secure(game_state: &GameState, target_card: u8, vox_payload: Option<VoxPayload>) -> GameState {
    let current_player = game_state.current_player.clone();
    let card = game_state.court[target_card as usize].clone();
    let new_game_state = game_state.clone();
    
    if card.controlled_by() == Some(current_player.clone()) {
        match card {
            CourtCard::VoxCard { vox, agents } => {
                let tropies: Vec<Trophy> = agents.iter()
                    .filter(|a| a.color != current_player)
                    .map(|a| Trophy{trophy_type: TrophyType::Agent, count: a.count, player: a.color.clone()})
                    .collect();
                let mut current_player_area = new_game_state.get_player_area(&current_player);
                current_player_area.add_trophies(tropies);

                return (vox.on_secure)(&new_game_state, vox_payload.expect("VoxPayload required"));
            },
            CourtCard::GuildCard { guild, agents } => {
                let tropies: Vec<Trophy> = agents.iter()
                    .filter(|a| a.color != current_player)
                    .map(|a| Trophy{trophy_type: TrophyType::Agent, count: a.count, player: a.color.clone()})
                    .collect();
                let current_player_area = new_game_state.get_player_area(&current_player);
                let combined_trophies = current_player_area.add_trophies(tropies);
                let new_guild_cards: Vec<Guild> = current_player_area.guild_cards.iter().cloned().chain(vec![guild.clone()]).collect();
                let new_players: HashMap<Color, PlayerArea> = new_game_state.players.iter().map(|(c,p)| if *c==current_player 
                                    { (c.clone(), PlayerArea{ 
                                        tropies: combined_trophies.clone(),
                                        guild_cards: new_guild_cards.clone(),
                                        ..p.clone()
                                    })}
                                    else {(c.clone(),p.clone())}
                                ).collect();
                return GameState {
                    players: new_players,
                    ..new_game_state.clone()
                };
            },
        }
    }
    else { panic!("Can only secure controlled Card") };
}

fn tax(game_state: &GameState, target_system: u8, target_player: Color) -> GameState {
    let system = &game_state.systems[target_system as usize];
    let tax_rival = target_player != game_state.current_player;

    let mut new_game_state =  game_state.clone();

    match system {
        System::Unused => panic!("Cannot tax Unused System"),
        System::Used {system_type, controlled_by, .. } => {
            if tax_rival {
                if controlled_by != &Some(game_state.current_player.clone()) {
                    panic!("Cannot tax a rival in a System controlled by another player");
                }
                let rivals_play_area = new_game_state.get_player_area(&target_player).change_reserve(&ReserveType::Agents, -1);
                let mut current_player_area = new_game_state.get_player_area(&game_state.current_player);
                current_player_area.add_trophies(vec![Trophy {
                    trophy_type: TrophyType::Agent,
                    count: 1,
                    player: target_player.clone(),
                }]);
                new_game_state.players.insert(target_player.clone(), rivals_play_area);
            }
            new_game_state.systems[target_system as usize] = new_game_state.systems[target_system as usize].use_building(&BuildingType::City, &target_player);
            
            let taxed_resource = match system_type {
                SystemType::Gate => panic!("Cannot tax Gate System"),
                SystemType::Planet { resource } => resource.clone(),
            };

            let resource_count = new_game_state.resource_reserve.get(&taxed_resource).expect("No Resource in Reserve").clone();
            if resource_count > 0 {
                new_game_state.resource_reserve.insert(taxed_resource.clone(), resource_count - 1);
                new_game_state.next_turn_states = vec![new_game_state.turn_state];
                new_game_state.turn_state = TurnState::AllocateResources {player: new_game_state.current_player.clone(), resources: vec![taxed_resource] };
            }
        }

    }
    new_game_state
}

fn declare_ambition(game_state: &GameState, ambition: AmbitionTypes) -> GameState {
    let highest_ambition = game_state.ambition_markers.iter().max_by_key(|am| if am.flipped {am.first_place_flipped} else {am.first_place});
    let highest_ambition = match highest_ambition {
        Some(a) => a,
        None => panic!("Cannot declare, because no AmbitionMarker is available")
    };
    let mut ambitions = game_state.ambitions.clone();
    let ambition_box = ambitions.get(&ambition).unwrap();
    ambitions.insert(ambition, Ambition{markers: ambition_box.markers.iter().chain(vec![highest_ambition]).cloned().collect(), .. ambition_box.clone()});
    GameState{
        ambition_markers: game_state.ambition_markers.iter().filter(|am| am != &highest_ambition).cloned().collect(),
        ambitions: ambitions,
        .. game_state.clone()
    }
}

fn execute_prelude_action(game_state: &GameState, action: BasicAction, resource: Option<ResourceType>) -> GameState {
    match (action.clone(), resource.clone().expect("No Resource in ResourceSlot"), game_state.lead_card.clone().unwrap().0.action_type) {
        (BasicAction::Build { target_system, build_type }, ResourceType::Material, _) => build(game_state, target_system, build_type),
        (BasicAction::Build { target_system, build_type }, ResourceType::Psionics, ActionType::Construction) => build(game_state, target_system, build_type),
        (BasicAction::Repair { target_system, build_type }, ResourceType::Material, _) => repair(game_state, target_system, build_type),
        (BasicAction::Repair { target_system, build_type }, ResourceType::Psionics, ActionType::Mobilization) => repair(game_state, target_system, build_type),
        (BasicAction::Tax { target_system, target_player }, ResourceType::Psionics, ActionType::Administration) => tax(game_state, target_system, target_player),
        (BasicAction::Influence { card_id }, ResourceType::Psionics, ActionType::Administration | ActionType::Mobilization) => influence(game_state, card_id),
        (BasicAction::Move { origin_id, destination_id, fresh_ships, damaged_ships }, ResourceType::Fuel, _) => move_ships(game_state, origin_id, destination_id, fresh_ships, damaged_ships),
        (BasicAction::Move { origin_id, destination_id, fresh_ships, damaged_ships }, ResourceType::Psionics, ActionType::Agression | ActionType::Mobilization) => move_ships(game_state, origin_id, destination_id, fresh_ships, damaged_ships),
        (BasicAction::Secure { card_id, vox_payload }, ResourceType::Relics, _) => secure(game_state, card_id, vox_payload),
        (BasicAction::Secure { card_id, vox_payload }, ResourceType::Psionics, ActionType::Agression) => secure(game_state, card_id, vox_payload),
        (BasicAction::Battle { target_system, target_player, dice }, ResourceType::Psionics, ActionType::Agression) => battle(game_state, target_system, target_player, dice),
        _ => panic!("Cannot execute {:?} with {:?} resource and {:?} lead", action.clone(), resource.clone(), game_state.lead_card.clone().unwrap().0.action_type)
    }
}

fn allocate_resources(game_state: &GameState, configuration: Vec<(u8, ResourceType)>) -> GameState {
    let (current_player, additional_resources) = match &game_state.turn_state {
        TurnState::AllocateResources { player, resources } => (player, resources),
        _ => panic!("Cannot allocate resources in {:?}", game_state.turn_state)
    };
    let current_resource_slots = game_state.players.get(&current_player).unwrap().resource_slots.clone();

    let available_resources: Vec<ResourceType> = current_resource_slots
        .iter()
        .filter_map(|r| {
                    match r {
                        ResourceSlot::Used { keys: _, resource } => Some(resource.clone()),
                        _ => None
                    }
                })
                .chain(additional_resources.iter().cloned())
                .collect();

    let available_resourceslots = current_resource_slots
        .iter()
        .filter(|s| match s {
            ResourceSlot::Used {..} | ResourceSlot::Unused {..} => true,
            _ => false 
        })
        .count();

    if configuration.len() > available_resourceslots {panic!("Cannot allocate {:?} resources into {:?} available ResourceSlots", configuration.len(), available_resourceslots)}

    if configuration.iter().map(|(i,_)|i).tuple_windows().any(|(i1,i2)| i1==i2) {panic!("Configuration includes index duplicates")}

    let config_resources = configuration
        .iter()
        .map(|(_,r)| r)
        .fold(
            vec![
            (ResourceType::Fuel, 0),
            (ResourceType::Material, 0),
            (ResourceType::Weapons, 0),
            (ResourceType::Relics, 0),
            (ResourceType::Psionics, 0),
            ]
            .into_iter()
            .collect::<HashMap<ResourceType, u8>>(),
            |mut acc, r| {
            *acc.entry(r.clone()).or_insert(0) += 1;
            acc
            },
        );
        
    let available_resources = available_resources
        .iter()
        .fold(
            vec![
            (ResourceType::Fuel, 0),
            (ResourceType::Material, 0),
            (ResourceType::Weapons, 0),
            (ResourceType::Relics, 0),
            (ResourceType::Psionics, 0),
            ]
            .into_iter()
            .collect::<HashMap<ResourceType, u8>>(),
            |mut acc, r| {
            *acc.entry(r.clone()).or_insert(0) += 1;
            acc
            },
        );

    if config_resources.iter().zip(available_resources.iter()).any(|((_, &config_count), (_, &available_count))| config_count > available_count) {
        panic!("Trying to allocate more resources than available");
    }

    let new_resource_slots: Vec<ResourceSlot> = current_resource_slots
        .iter()
        .zip(
            (0..current_resource_slots.len()).map(|i| {
                configuration.iter()
                    .find(|(idx, _)| *idx as usize == i)
                    .map(|(_, r)| r.clone())
            })
        )
        .map(|(s,r)| {
            match (s,r) {
                (ResourceSlot::Used { keys, ..} | ResourceSlot::Unused { keys } , None) => ResourceSlot::Unused { keys: *keys },
                (ResourceSlot::Used { keys, ..} | ResourceSlot::Unused { keys }, Some(resource)) => ResourceSlot::Used { keys: *keys, resource: resource },
                (ResourceSlot::Covered { keys }, None) => ResourceSlot::Covered { keys: *keys },
                (ResourceSlot::Covered { ..}, Some(_)) => panic!("Cannot allocate Resource in Covered ResourceSlot"),
            }
        })
        .collect();

    let new_players: HashMap<Color, PlayerArea> = game_state.players
            .iter()
            .map(|(c,p)|if c == current_player {(c.clone(),PlayerArea{resource_slots: new_resource_slots.clone(), ..p.clone()})} else {(c.clone(),p.clone())})
            .collect();

    let overflow_resources: Vec<ResourceType> = available_resources.iter()
        .flat_map(|(resource, &available_count)| {
            let used_count = config_resources.get(resource).cloned().unwrap_or(0);
            if available_count > used_count {
                std::iter::repeat(resource.clone()).take((available_count - used_count) as usize).collect::<Vec<_>>()
            } else {
                Vec::new()
            }
        })
        .collect();

    let new_resource_reserve: HashMap<ResourceType,u8> = game_state.resource_reserve
        .iter()
        .map(|(r,c)|{
            let overflow = overflow_resources.iter().filter(|res| res == &r).count();
            (r.clone(), c+overflow as u8)
        })
        .collect();

    GameState {
        players: new_players,
        resource_reserve: new_resource_reserve,
        .. game_state.clone()
    }
}

fn end_chapter(game_state: &GameState) -> GameState {
    //Evaluate Ambitions
    println!("End Chapter");
    game_state.clone()
}

fn end_round(game_state: &GameState) -> GameState {
    println!("Ending Round");
    //determine new Initiative, discard Cards
    let mut new_game_state = game_state.clone();

    let lead = game_state.lead_card.as_ref().unwrap();
    let follow_cards = game_state.follow_cards.clone();

    new_game_state.initiative =  match new_game_state.seized.clone() {
        Some(c) => c,
        None => {
            game_state.follow_cards.iter().chain(vec![lead]).filter(|(c, f, _)| c.action_type == lead.0.action_type && *f).max_by_key(|(c, _, _)| c.number).unwrap().2.clone()
        }
    };

    new_game_state.current_player = new_game_state.initiative.clone();

    new_game_state.action_discard.push(lead.0.clone());
    new_game_state.action_discard = new_game_state.action_discard.iter().cloned().chain(follow_cards.iter().map(|(c, _, _)| c.clone())).collect();
    new_game_state.follow_cards = vec![];
    new_game_state.turn_state = TurnState::TrickTaking;
    new_game_state.players_in_round = new_game_state.players.iter().filter(|(_, a)| a.action_cards.len() != 0).count() as u8;

    if new_game_state.players.iter().filter(|(_, area)| area.action_cards.len() != 0).count() == 0 {end_chapter(&new_game_state)} else {new_game_state}
}

fn end_turn(game_state: &GameState) -> GameState {
    //Otherwise next players Turn
    let player_order: Vec<Color> = vec![Color::Red, Color::Blue, Color::White, Color::Yellow].iter().take(game_state.players.len()).cloned().collect();
    let mut new_game_state = game_state.clone();
    new_game_state.turn_state = TurnState::TrickTaking;
    new_game_state.current_player = player_order[(player_order.iter().position(|c| *c == new_game_state.current_player).unwrap() + 1)%player_order.len()].clone();
    new_game_state.systems = new_game_state.systems.iter().map(|s| s.refresh_buildings()).collect();

    // last player in Turn Order
    if (1 + game_state.follow_cards.len()) as u8 == game_state.players_in_round {
        return end_round(&new_game_state);
    } else {
        new_game_state
    }
}

fn play_lead_card(game_state: &GameState, card: ActionCard, declare: Option<AmbitionTypes>) -> GameState {
    let mut new_game_state = game_state.clone();
    let mut player_area = new_game_state.get_player_area(&new_game_state.current_player);
    player_area.remove_action_card(card.clone());
    new_game_state.players.insert(new_game_state.current_player.clone(), player_area);
    new_game_state.lead_card = Some((card.clone(), true, game_state.current_player.clone()));
    new_game_state.turn_state = TurnState::Prelude { action_type: card.action_type.clone(), pips_left: card.pips };
    match declare {
        Some(ambition) => {
            match &card.declared_ambition {
                Some(possible_declare) => if possible_declare == &ambition 
                        {declare_ambition(game_state, ambition)}
                    else
                        {panic!("AmbitionType on card {:?} does not match Ambition {:?}", &card, ambition)},
                None => panic!("Cannot declare Ambition with {:?}", card),
            }
        },
        None => new_game_state 
    }
}

fn surpass(game_state: &GameState, card: ActionCard, seize: Option<ActionCard>) -> GameState {
    let (lead_card, _, _) = game_state.lead_card.as_ref().expect("Cannot Surpass, if no lead card exists");
    if card.action_type != lead_card.action_type {panic!("Cannot surpass with other card type")}
    if card.number < lead_card.number {panic!("Cannot surpass with a lower card")};
    let mut new_game_state = game_state.clone();
    let mut player_area = new_game_state.get_player_area(&new_game_state.current_player);
    player_area.remove_action_card(card.clone());
    new_game_state.players.insert(new_game_state.current_player.clone(), player_area);
    new_game_state.follow_cards.push((card.clone(), true, game_state.current_player.clone()));
    match seize {
        Some(_) => todo!(),
        None => {},
    }
    new_game_state.turn_state = TurnState::Prelude { action_type: card.action_type, pips_left: card.pips };
    new_game_state
}

fn copy(game_state: &GameState, card: ActionCard, seize: Option<ActionCard>) -> GameState {
    let (lead_card, _, _) = game_state.lead_card.as_ref().unwrap();
    let mut new_game_state = game_state.clone();
    let mut player_area = new_game_state.get_player_area(&new_game_state.current_player);
    player_area.remove_action_card(card.clone());
    new_game_state.players.insert(new_game_state.current_player.clone(), player_area);
    new_game_state.follow_cards.push((card.clone(), false, game_state.current_player.clone()));
    match seize {
        Some(_) => new_game_state.seized = Some(new_game_state.current_player.clone()),
        None => {},
    }
    new_game_state.turn_state = TurnState::Prelude { action_type: lead_card.action_type.clone(), pips_left: 1 };
    new_game_state
}

fn pivot(game_state: &GameState, card: ActionCard, seize: Option<ActionCard>) -> GameState {
    let (lead_card, _, _) = game_state.lead_card.as_ref().unwrap();
    if card.action_type == lead_card.action_type {panic!("Cannot Pivot with same card type")};
    let mut new_game_state = game_state.clone();
    let mut player_area = new_game_state.get_player_area(&new_game_state.current_player);
    player_area.remove_action_card(card.clone());
    new_game_state.players.insert(new_game_state.current_player.clone(), player_area);
    new_game_state.follow_cards.push((card.clone(), true, game_state.current_player.clone()));
    match seize {
        Some(_) => new_game_state.seized = Some(new_game_state.current_player.clone()),
        None => {},
    }
    new_game_state.turn_state = TurnState::Prelude { action_type: card.action_type, pips_left: 1 };
    new_game_state
}

pub fn execute_actions(game_state: &GameState, actions: Vec<Action>) -> GameState {
    return actions
        .iter()
        .fold(game_state.clone(), |g, action| execute_action(&g, action.clone()));
}

pub fn execute_action(game_state: &GameState, action: Action) -> GameState {
    match &game_state.turn_state {
        TurnState::TrickTaking => {
            match action {
                Action::PlayLeadCard { card, declare } => play_lead_card(game_state, card, declare),
                Action::Surpass { card, seize } => surpass(game_state, card, seize),
                Action::Copy { card, seize } => copy(game_state, card, seize),
                Action::Pivot { card, seize } => pivot(game_state, card, seize),
                _ => panic!("Can only Execute TrickTaking Actions")
            }
        },
        TurnState::Prelude { action_type, pips_left } => {
            match action {
                Action::EndPrelude => {
                    let mut new_game_state = game_state.clone();
                    new_game_state.turn_state = TurnState::Actions { action_type: action_type.clone(), pips_left: pips_left.clone() };
                    new_game_state
                },
                Action::PreludeResourceAction { basic_action, used_resource } => execute_prelude_action(
                    game_state,
                    basic_action,
                    game_state.players.get(&game_state.current_player).unwrap().get_resource(used_resource)
                ),
                Action::UseWeapons { used_resource } => todo!(),
                Action::PreludeCard { guild_card, prelude_action_payload } => todo!(),
                _ => panic!("Cannot execute {:?} in Prelude", action)
            }
        },
        TurnState::Actions { action_type, pips_left } => {
            if action == Action::EndTurn {return end_turn(game_state)}
            if *pips_left == 0 {panic!("No Action pips left in {:?}, when executing {:?}", game_state.turn_state, action)}
            return match (action_type, action) {
                (ActionType::Administration, Action::MainAction { basic_action }) => match basic_action {
                    BasicAction::Repair { target_system, build_type } => repair(&use_action_pip(&game_state), target_system, build_type),
                    BasicAction::Tax { target_system, target_player } => tax(&use_action_pip(&game_state), target_system, target_player),
                    BasicAction::Influence { card_id } => influence(&use_action_pip(&game_state), card_id),
                    _ => panic!("Cannot execute Action with Administration Action Card")                    
                },
                (ActionType::Agression, Action::MainAction { basic_action }) => match basic_action {
                    BasicAction::Move { origin_id, destination_id, fresh_ships, damaged_ships } => move_ships(&use_action_pip(&game_state), origin_id, destination_id, fresh_ships, damaged_ships),
                    BasicAction::Secure { card_id , vox_payload} => secure(&use_action_pip(&game_state), card_id, vox_payload),
                    BasicAction::Battle { target_system, target_player, dice } => battle(&use_action_pip(&game_state), target_system, target_player, dice),
                    _ => panic!("Cannot execute Action with Aggresion Action Card")
                },
                (ActionType::Construction, Action::MainAction { basic_action }) => match basic_action {
                    BasicAction::Build { target_system, build_type } => build(&use_action_pip(game_state), target_system, build_type),
                    BasicAction::Repair { target_system, build_type } => repair(&use_action_pip(&game_state), target_system, build_type),
                    _ => panic!("Cannot execute Action with Aggresion Action Card")                    
                },
                (ActionType::Mobilization, Action::MainAction { basic_action }) => match basic_action {
                    BasicAction::Move { origin_id, destination_id, fresh_ships, damaged_ships } => move_ships(&use_action_pip(&game_state), origin_id, destination_id, fresh_ships, damaged_ships),
                    BasicAction::Influence { card_id } => influence(&use_action_pip(&game_state), card_id),
                    _ => panic!("Cannot execute Action with Mobilization Action Card")                    
                },
                (_ , Action::EndTurn) => todo!(),
                _ => panic!("")
            }
        }
        TurnState::AllocateResources { ..} => match action {
            Action::AllocateResources { configuration } => allocate_resources(game_state, configuration),
            _ => panic!("Can only AllocateResources in the AllocateResources Turnstate not {:?}", action)
        },
        TurnState::AllocateDiceResults { target_system, target_player, self_hits, hits, building_hits, keys } => todo!(),
    }
}