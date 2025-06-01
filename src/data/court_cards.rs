use super::game_state::{GameState, Color, Agents, ResourceType, AmbitionTypes, PreludeActionPayload};

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