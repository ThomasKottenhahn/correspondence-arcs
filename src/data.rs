#![allow(dead_code)]

use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Color {
    None,
    Red,
    Blue,
    White,
    Yellow
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ships {
    pub player: Color,
    pub fresh: u8,
    pub damaged: u8
}

#[derive(Clone, Debug, PartialEq)]
pub enum BuildingType {
    City,
    Starport
}

#[derive(Clone, Debug, PartialEq)]
pub enum BuildingSlot {
    Occupied {fresh: bool, player: Color, building_type: BuildingType, used: bool},
    Empty
}

#[derive(Clone, Debug)]
pub enum SystemType{
    Gate,
    Planet {resource: ResourceType}
}

#[derive(Clone, Debug)]
pub enum System {
    Unused,
    Used {
    system_id: u8,
    system_type: SystemType,
    building_slots: Vec<BuildingSlot>,
    ships: Vec<Ships>,
    controlled_by: Option<Color>,
    connects_to: Vec<u8>
    }
}

impl System{
    pub fn has_presence(self: &System, player_color: &Color) -> bool {
        match self {
            System::Unused => false,
            System::Used {ships, ..} => ships.iter().filter(|x| x.player==*player_color && (x.fresh > 0 || x.damaged > 0)).count() == 1
        }
    }

    pub fn update_control(self: &System) -> System {
        match self {
            System::Unused => self.clone(),
            System::Used {
                system_id,
                system_type,
                building_slots,
                ships,
                connects_to, ..
            } => {
                let (new_controlled_by, _) = ships.iter().fold((None, 0), |acc: (Option<Color>, u8), s| {
                    match acc {
                        (None, max_count) => if s.fresh > max_count {(Some(s.player.clone()), s.fresh)} else {(None, max_count)},
                        (Some(ref max_player), max_count) => {
                            if s.fresh > max_count {
                                (Some(s.player.clone()), s.fresh)
                            } else if s.fresh == max_count && s.player != *max_player && s.fresh != 0 {
                                (None, max_count)
                            } else {
                                (Some(max_player.clone()), max_count)
                            }
                        }
                    }
                });
                return System::Used {
                    system_id: system_id.clone(),
                    system_type: system_type.clone(),
                    building_slots: building_slots.clone(),
                    ships: ships.clone(),
                    controlled_by: new_controlled_by,
                    connects_to: connects_to.clone()
                };
            }
        }
    }   

    pub fn get_fresh_ships(self: &System, color: &Color) -> u8 {
        match self {
            System::Unused => 0,
            System::Used { ships, .. } => {
                ships
                    .iter()
                    .find(|s| s.player == *color)
                    .map_or(0, |s| s.fresh)
            }
        }
    }

    pub fn has_unused_building(self:&System, building_type: BuildingType, color: &Color) -> Option<u8> {
        match self {
            System::Unused => None,
            System::Used { building_slots, .. } => building_slots
            .iter()
            .enumerate()
            .find_map(|(idx, b)| {
                match b {
                BuildingSlot::Occupied { fresh, player, building_type: b_type, used } => {
                    if *fresh && player == color && *b_type == building_type && !*used {
                    Some(idx as u8)
                    } else {
                    None
                    }
                }
                BuildingSlot::Empty => None,
                }
            })
        }
    }

}

#[derive(Clone, Debug)]
pub struct Cluster {
    gate_id: u8,
    systems_id: Vec<u8>
}

pub struct SetupCard {
    pub players: u8,
    pub cluster_out_of_play: Vec<u8>,
    pub a_locations: Vec<u8>,
    pub b_locations: Vec<u8>,
    pub c_locations: Vec<u8>
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResourceType {
    Fuel,
    Material,
    Weapons,
    Relics,
    Psionics
}
#[derive(Clone, Debug)]
pub enum CourtCard {
    VoxCard {vox: Vox, agents: Vec<Agents> },
    GuildCard {guild: Guild, agents: Vec<Agents>}
}

impl CourtCard {
    pub fn controlled_by(&self) -> Option<Color> {
        match self {
            CourtCard::VoxCard { agents, .. } | CourtCard::GuildCard { agents, .. } => {
                agents.iter().fold(None, |acc: Option<(Color, u8)>, agent| {
                    match acc {
                        None => Some((agent.color.clone(), agent.count)),
                        Some((ref max_color, max_count)) => {
                            if agent.count > max_count {
                                Some((agent.color.clone(), agent.count))
                            } else if agent.count == max_count && agent.color != *max_color && agent.count != 0 {
                                None
                            } else {
                                Some((max_color.clone(), max_count))
                            }
                        }
                    }
                }).map(|(color, _)| color)
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Vox {
    pub title: String,
    pub description: String,
    pub on_secure: fn(&GameState, VoxPayload) -> GameState
}
    
#[derive(Clone, Debug)]
pub struct Guild {
    title: String,
    description: String,
    resource: ResourceType,
    keys: u8,
    prelude_action: fn(&GameState, PreludeActionPayload) -> GameState
}

#[derive(Clone, Debug)]
pub enum VoxPayload {
    MassUprising {target_cluster: u8},
    PopulistDemands {ambition: Option<AmbitionTypes>},
    OutrageSpreads {outrage_type: Option<ResourceType>},
    SongOfFreedom {target_system: Option<u8>, target_player: Option<Color>},
    GuildStruggle {target_player: Option<Color>, target_card: Option<u8>},
    CallToAction
}

pub enum PreludeActionPayload {
    Interest {target_resource: ResourceType},
    Steal {target_resource: ResourceType, target_player: ResourceType},
    Union {card_type: ActionType},
    PlaceShips {target_system: u8},
    Farseers {cards: Vec<ActionCard>},
    RelicFence {target_resource: ResourceType},
    SilverTounges,
    ElderBroker,
    GateKeepers
}

#[derive(Clone, Debug)]
pub enum ResourceSlot {
    Used {keys: u8, resource: ResourceType},
    Unused {keys: u8},
    Covered
}

#[derive(Clone, Debug)]
pub struct Agents{
    pub color: Color,
    pub count: u8
}

#[derive(Clone, Debug, PartialEq)]
pub enum TrophyType {
    Ship,
    Building,
    Agent
}

#[derive(Clone, Debug)]
pub struct Trophy {
    pub trophy_type: TrophyType,
    pub count: u8,
    pub player: Color
}

#[derive(Clone, Debug)]
pub enum BuildType{
    Starport,
    City,
    Ship
}

#[derive(Clone, Debug)]
pub enum Dice{
    Skirmish,
    Assault,
    Raid
}

#[derive(Clone, Debug)]
pub enum Action{
    PlayLeadCard {card: ActionCard, declare: Option<AmbitionTypes>},
    Surpass {card: ActionCard, seize: bool},
    Copy {card: ActionCard, seize: bool},
    Pivot {card: ActionType, seize: bool},
    Build {target_system: u8, build_type: BuildType},
    Repair {target_system: u8, build_type: BuildType},
    Tax {target_system: u8, target_player: Color},
    Influence {card_id: u8},
    Move {origin_id: u8, destination_id: u8, fresh_ships: u8, damaged_ships: u8},
    Secure {card_id: u8, vox_payload: Option<VoxPayload>},
    Battle {target_system: u8, target_player: Color, dice: Vec<Dice>},
    EndPrelude,
    Pass
}

#[derive(Clone, Debug)]
pub enum ActionType{
    Administration,
    Agression,
    Construction,
    Mobilization
}

#[derive(Clone, Debug)]
pub struct ActionCard{
    pub action_type: ActionType,
    pub number: u8,
    pub pips: u8,
    pub declared_ambition: Option<AmbitionTypes>
}

#[derive(Clone, Debug)]
pub struct PlayerArea {
    pub player: Color,
    pub initiative: bool,
    pub action_cards: Vec<ActionCard>,
    pub guild_cards: Vec<Guild>,
    pub reserve_ships: u8,
    pub reserve_agents: u8,
    pub reserve_starports: u8,
    pub reserve_cities: u8,
    pub resource_slots: Vec<ResourceSlot>,
    pub captives: Vec<Agents>,
    pub tropies: Vec<Trophy>
}

impl PlayerArea {
    pub fn add_trophies(& mut self, tropies: Vec<Trophy>){
        let combined = tropies.iter().fold(self.tropies.clone(), {
            |mut acc, trophy| {
                if let Some(existing) = acc.iter_mut().find(|t| t.trophy_type == trophy.trophy_type && t.player == trophy.player) {
                    existing.count += trophy.count;
                } else {
                    acc.push(trophy.clone());
                }
                acc
            }
        });
        self.tropies = combined;
    }
}

#[derive(Clone, Debug)]
struct AmbitionMarker{
    first_place: u8,
    second_place: u8,
    flipped: bool,
    first_place_flipped: u8,
    second_place_flipped: u8
}

#[derive(Clone, Debug)]
pub enum AmbitionTypes {
    Tycoon,
    Tyrant,
    Warlord,
    Keeper,
    Empath
}

#[derive(Clone, Debug)]
pub struct Ambition{
    ambition_type: AmbitionTypes,
    markers: Vec<AmbitionMarker>,
    discarded_resources: Vec<ResourceType>
}

#[derive(Clone, Debug)]
pub enum TurnState {
    TrickTaking,
    Prelude {action_type: ActionType, pips_left: u8},
    Actions {action_type: ActionType, pips_left: u8},
    AllocateResource {resource: ResourceType},
    AllocateDiceResults {target_system: u8, target_player: Color, self_hits: u8, hits: u8, building_hits: u8, keys: u8}
}

#[derive(Clone, Debug)]
pub struct GameState {
    pub players: Vec<PlayerArea>,
    pub current_player: Color,
    pub initiative: Color,
    pub seized: bool,
    pub zero_marker: bool,
    pub turn_state: TurnState,
    pub next_turn_state: Option<TurnState>,
    pub chapter: u8,
    pub systems: Vec<System>,
    pub resource_reserve: HashMap<ResourceType, u8>,
    pub court: Vec<CourtCard>,
    pub court_draw_pile: Vec<CourtCard>,
    pub court_discard_pile: Vec<CourtCard>,
    pub action_discard: Vec<ActionCard>,
    pub lead_card: Option<ActionCard>,
    pub follow_cards: Vec<ActionCard>
}

impl GameState {
    pub fn get_player_area(&self, color: &Color) -> PlayerArea {
        self.players.iter().find(|p| p.player == *color).expect("No such player exists").clone()
    }

    pub fn get_player_area_ind(&self, color: &Color) -> u8 {
        self.players.iter().position(|p| p.player == *color).expect("No such player exists") as u8
    }
}