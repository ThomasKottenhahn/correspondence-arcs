use super::game_state::{Color, ResourceType};

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

#[derive(Clone, Debug, PartialEq)]
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

    pub fn get_all_ships(self:&System, color: &Color) -> u8 {
        match self {
            System::Unused => 0,
            System::Used { ships, .. } => {
                ships
                    .iter()
                    .find(|s| s.player == *color)
                    .map_or(0, |s| s.fresh + s.damaged)
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