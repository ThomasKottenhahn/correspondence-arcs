#![allow(dead_code)]
#![allow(unused_variables)]

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

pub fn create_reach(setup_card: &SetupCard) -> Vec<System> {
    let mut systems:Vec<System> = vec![];

    for i in 0..6 {
        let gate ={
            let mut connections = vec![];
            let mut j = 5;
            while setup_card.cluster_out_of_play.contains(&((i+j)%6)){
                j = j-1
            }
            connections.push((i+j)%6);

            let mut j = 1;
            while setup_card.cluster_out_of_play.contains(&((i+j)%6)){
                j = j+1
            }
            connections.push((i+j)%6);

            if setup_card.cluster_out_of_play.contains(&i) {
                System::Unused
            }else{
                System::Used {
                system_id: i,
                system_type: SystemType::Gate,
                building_slots: vec![],
                ships: vec![],
                controlled_by: None,
                connects_to: connections
            }
        }
        };
        systems.push(gate);
    }
    let resource_types = vec![ResourceType::Weapons, ResourceType::Fuel, ResourceType::Material, ResourceType::Psionics, ResourceType::Weapons, ResourceType::Relics, ResourceType::Material, ResourceType::Fuel, ResourceType::Weapons, ResourceType::Relics, ResourceType::Fuel, ResourceType::Material, ResourceType::Weapons, ResourceType::Relics, ResourceType::Psionics, ResourceType::Material, ResourceType::Fuel, ResourceType::Psionics];
    let building_slots_nr = vec![2,1,2,1,1,2,1,1,2,2,2,1,1,1,1,1,2,1];
    let empty_building_slot = BuildingSlot::Empty;

    for i in 0..18{
        let planet = {
            if setup_card.cluster_out_of_play.contains(&(((i/3)as u8))){
                System::Unused
            }else{
                let systems_building_slots = {
                    if building_slots_nr[i] == 1{
                        vec![empty_building_slot.clone()]
                    } else{
                        vec![empty_building_slot.clone(), empty_building_slot.clone()]
                    }
                };
                
                let mut connections = vec![(i/3) as u8];
                if i != 0 {if i/3 == (i-1)/3 {connections.push((i+5) as u8)}}
                if i/3 == (i+1)/3 {connections.push((i+7) as u8)}
                
                if (i == 5 || i == 14) && !setup_card.cluster_out_of_play.contains(&((i/3)as u8)) {connections.push((i+7) as u8)}
                if (i == 6 || i == 15) && !setup_card.cluster_out_of_play.contains(&((i/3)as u8)) {connections.push((i+5) as u8)}
                
                System::Used {
                    system_id: (6+i) as u8,
                    system_type: SystemType::Planet {resource: resource_types[i].clone()},
                    building_slots: systems_building_slots,
                    ships: vec![],
                    controlled_by: None,
                    connects_to: connections
                }
        
            }
        };
        systems.push(planet);
    }
    

    for i in 0..systems.len() {
        println!("{:?}",systems[i]);
    }
    return systems;
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

enum AmbitionTypes {
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