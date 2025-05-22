#![allow(dead_code)]

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
    ships: Vec<Ships>,
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
    pub cluster_out_of_play: Vec<u8>,
    pub a_locations: Vec<u8>,
    pub b_locations: Vec<u8>,
    pub c_locations: Vec<u8>
}

#[derive(Clone, Debug)]
pub enum ResourceType {
    Fuel,
    Material,
    Weapons,
    Relics,
    Psionics
}

#[derive(Clone, Debug)]
pub enum CourtCard {
    Vox {
        title: String,
        description: String,
        on_secure: fn(&GameState, VoxPayload) -> GameState
    },
    Guild {
        title: String,
        description: String,
        resource: ResourceType,
        keys: u8,
        prelude_action: fn(&GameState, PreludeActionPayload) -> GameState
    }
}

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
pub struct Agent{color: Color}

#[derive(Clone, Debug)]
pub enum TrophyType {
    Ship,
    Building,
    Agent
}

#[derive(Clone, Debug)]
pub struct Trophy {
    trophy_type: TrophyType,
    player: Color
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
    Build {target_system: u8, build_type: BuildType},
    Repair {target_system: u8, build_type: BuildType},
    Tax {target_system: u8, resource: ResourceType},
    Influence {card_id: u8},
    Move {origin_id: u8, destination_id: u8, fresh_ships: u8, damaged_ships: u8},
    Secure {card_id: u8},
    Battle {target_system: u8, target_player: Color, dice: Vec<Dice>}
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
    action_type: ActionType,
    number: u8,
    pips: u8,
    declared_ambition: Option<AmbitionTypes>
}

#[derive(Clone, Debug)]
pub struct PlayerArea {
    pub player: Color,
    pub initiative: bool,
    pub action_cards: Vec<ActionCard>,
    pub controlled_systems: Vec<System>,
    pub controlled_ships: Vec<Ships>,
    pub controlled_buildings: Vec<BuildingSlot>,
    pub guild_cards: Vec<CourtCard>,
    pub reserve_ships: u8,
    pub reserve_agents: u8,
    pub reserve_starports: u8,
    pub reserve_cities: u8,
    pub resource_slots: Vec<ResourceSlot>,
    pub captives: Vec<Agent>,
    pub tropies: Vec<Trophy>
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
struct Ambition{
    ambition_type: AmbitionTypes,
    markers: Vec<AmbitionMarker>,
    discarded_resources: Vec<ResourceType>
}

#[derive(Clone, Debug)]
pub enum TurnState {
    TrickTaking,
    Prelude,
    Actions {action_type: ActionType, pips_left: u8},
    AllocateResource {resource: ResourceType},
    AllocateDiceResults {target_system: u8, opponent: Color, self_hits: u8, hits: u8, building_hits: u8, keys: u8}
}

#[derive(Clone, Debug)]
pub struct GameState {
    pub players: Vec<PlayerArea>,
    pub current_player: Color,
    pub turn_state: TurnState,
    pub chapter: u8,
    pub systems: Vec<System>,
    pub court: Vec<CourtCard>,
    pub court_draw_pile: Vec<CourtCard>,
    pub court_discard_pile: Vec<CourtCard>,
    pub action_discard: Vec<ActionCard>,
    pub lead_card: Option<ActionCard>,
    pub follow_cards: Vec<ActionCard>
}