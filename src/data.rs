#![allow(dead_code)]

#[derive(Clone, Debug)]
pub enum Color {
    Red,
    Blue,
    White,
    Yellow
}

#[derive(Clone, Debug)]
pub struct Ship {
    fresh: bool,
    player: Color
}

#[derive(Clone, Debug)]
pub enum BuildingType {
    City,
    Starport
}

#[derive(Clone, Debug)]
pub enum BuildingSlot {
    Occupied {fresh: bool, player: Color, building_type: BuildingType},
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
    ships: Vec<Ship>,
    controlled_by: Option<Color>,
    connects_to: Vec<u8>
    }
}

#[derive(Clone, Debug)]
pub struct Cluster {
    gate_id: u8,
    systems_id: Vec<u8>
}

pub struct SetupCard {
    pub players: u8,
    pub cluster_out_of_play: Vec<u8>
}

#[derive(Clone, Debug)]
pub enum ResourceType {
    Fuel,
    Material,
    Weapons,
    Relics,
    Psionics
}

enum CourtCard {
    Vox {
        title: String,
        description: String,
        on_secure: fn(&GameState) -> GameState
    },
    Guild {
        title: String,
        description: String,
        resource: ResourceType,
        keys: u8,
        prelude_action: fn(&GameState) -> GameState
    }
}

enum ResourceSlot {
    Used {keys: u8, resource: ResourceType},
    Unused {keys: u8},
    Covered
}

struct Agent{color: Color}

enum TrophyType {
    Ship,
    Building,
    Agent
}

struct Trophy {
    trophy_type: TrophyType,
    player: Color
}

enum BuildType{
    Starport,
    City,
    Ship
}

enum Dice{
    Skirmish,
    Assault,
    Raid
}

enum Action{
    Build {target_system: u8, build_type: BuildType},
    Repair {target_system: u8, build_type: BuildType},
    Tax {target_system: u8, resource: ResourceType},
    Influence {card_id: u8},
    Move {origin_id: u8, destination_id: u8, fresh_ships: u8, damaged_ships: u8},
    Secure {card_id: u8},
    Battle {target_system: u8, target_player: Color, dice: Vec<Dice>}
}

pub enum ActionType{
    Administration,
    Agression,
    Construction,
    Mobilization
}

pub struct ActionCard{
    action_type: ActionType,
    number: u8,
    pips: u8,
    declared_ambition: Option<AmbitionTypes>
}

struct PlayerArea {
    player: Color,
    initiative: bool,
    action_cards: Vec<ActionCard>,
    controlled_systems: Vec<System>,
    controlled_ships: Vec<Ship>,
    controlled_buildings: Vec<BuildingSlot>,
    guild_cards: Vec<CourtCard>,
    reserve_ships: u8,
    reserve_agents: u8,
    reserve_starports: u8,
    reserve_cities: u8,
    resource_slots: Vec<ResourceSlot>,
    captives: Vec<Agent>,
    tropies: Vec<Trophy>
}

struct AmbitionMarker{
    first_place: u8,
    second_place: u8,
    flipped: bool,
    first_place_flipped: u8,
    second_place_flipped: u8
}

pub enum AmbitionTypes {
    Tycoon,
    Tyrant,
    Warlord,
    Keeper,
    Empath
}

struct Ambition{
    ambition_type: AmbitionTypes,
    markers: Vec<AmbitionMarker>
}

enum TurnState {
    TrickTaking,
    Prelude,
    Actions {action_type: ActionType, pips_left: u8}
}

struct GameState {
    players: Vec<PlayerArea>,
    current_player: Color,
    turn_state: TurnState,
    chapter: u8,
    systems: Vec<System>,
    court: Vec<CourtCard>,
    court_draw_pile: Vec<CourtCard>,
    dourt_discard_pile: Vec<CourtCard>,
    action_discard: Vec<ActionCard>,
    lead_card: Option<ActionCard>,
    follow_cards: Vec<ActionCard>
}