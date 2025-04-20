#![allow(dead_code)]
#![allow(unused_variables)]

enum Color {
    Red,
    Blue,
    White,
    Yellow
}

struct Ship {
    fresh: bool,
    player: Color
}

enum BuildingType {
    City,
    Starport
}

enum BuildingSlot {
    Occupied {fresh: bool, player: Color, building_type: BuildingType},
    Empty
}

enum SystemType{
    Gate,
    Planet {resource: ResourceType}
}

struct System {
    system_type: SystemType,
    building_slots: Vec<BuildingSlot>,
    ships: Vec<Ship>,
    controlled_by: Option<Color>,
    connects_to: Vec<System>
}

struct Cluster {
    number: u8,
    gate: System,
    systems: Vec<System>
}

enum ResourceType {
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
        on_secure: fn()
    },
    Guild {
        title: String,
        description: String,
        resource: ResourceType
    }
}

enum ResourceSlot {
    Used {keys: u32, resource: ResourceType},
    Unused {keys: u32},
    Covered
}

struct Agent{color: Color}

enum TrophyType {
    Ship,
    Building,
    Agent
}

enum ActionType{
    Build,
    Repair,
    Tax,
    Influence,
    Move,
    Secure,
    Battle
}

struct ActionCard{
    action_types: Vec<ActionType>,
    number: u8,
    pips: u8,
    declared_ambition: Option<AmbitionTypes>
}

struct PlayerArea {
    player: Color,
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
    tropies: Vec<TrophyType>
}

enum AmbitionTypes {
    Tycoon,
    Tyrant,
    Warlord,
    Keeper,
    Empath
}

struct AmbitionMarker{
    first_place: u8,
    second_place: u8,
    flipped: bool,
    first_place_flipped: u8,
    second_place_flipped: u8
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
    chapter: u8,
    cluster: Vec<Cluster>,
    court: Vec<CourtCard>,
    court_draw_pile: Vec<CourtCard>,
    dourt_discard_pile: Vec<CourtCard>,
    action_discard: Vec<ActionCard>,
    lead_card: Option<ActionCard>,
    follow_cards: Vec<ActionCard>
}