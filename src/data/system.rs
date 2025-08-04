use std::collections::HashMap;

use super::game_state::{Color, ResourceType};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ships {
    pub fresh: u8,
    pub damaged: u8
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BuildingType {
    City,
    Starport
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BuildingSlot {
    Occupied {fresh: bool, player: Color, building_type: BuildingType, used: bool},
    Empty
}

impl BuildingSlot {
    pub fn use_building(&self) -> BuildingSlot {
        match self {
            BuildingSlot::Empty => panic!("Cannot use Empty BuildingSlot"),
            BuildingSlot::Occupied { fresh, player, building_type, used: false } => {
                BuildingSlot::Occupied {
                    fresh: *fresh, player: player.clone(), building_type: building_type.clone(), used: true,}
            },
            _ => panic!("Cannot use already used BuildingSlot")
        }
    }

    pub fn refresh_building(&self) -> BuildingSlot {
        match self {
            BuildingSlot::Empty => BuildingSlot::Empty,
            BuildingSlot::Occupied { fresh, player, building_type, used: _ } => {
                BuildingSlot::Occupied {
                    fresh: *fresh, player: player.clone(), building_type: building_type.clone(), used: false,}
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SystemType{
    Gate,
    Planet {resource: ResourceType}
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum System {
    Unused,
    Used {
    system_id: u8,
    system_type: SystemType,
    building_slots: Vec<BuildingSlot>,
    ships: HashMap<Color,Ships>,
    controlled_by: Option<Color>,
    connects_to: Vec<u8>
    }
}

impl System {
    pub fn use_building(&self, building: &BuildingType, player_color: &Color) -> System {
        match self {
            System::Unused => panic!("Cannot use Building in Unused System"),
            System::Used { system_id, system_type, building_slots, ships, controlled_by, connects_to } => {
                let building_position = building_slots.iter().position(|b| {
                    match b {
                        BuildingSlot::Occupied { fresh: _, player, building_type, used: false } => player == player_color && building_type == building,
                        _ => false
                    }
                });
                let building_position = match building_position {
                    Some(i) => i,
                    None => panic!("Cannot find unused Buildingslot in System {:?} of type {:?}", system_id, building)
                };
                System::Used { system_id: *system_id,
                    system_type: system_type.clone(),
                    building_slots: building_slots.iter().enumerate().map(|(i,b)| if i == building_position {b.use_building()} else {b.clone()}).collect(),
                    ships: ships.clone(),
                    controlled_by: controlled_by.clone(),
                    connects_to: connects_to.clone() 
                }
            },
        }
    }

    pub fn refresh_buildings(&self) -> System {
        match self {
            System::Unused => System::Unused,
            System::Used { system_id, system_type, building_slots, ships, controlled_by, connects_to } => {
                System::Used { 
                    system_id: *system_id,
                    system_type: system_type.clone(),
                    building_slots: building_slots.iter().map(|b| b.refresh_building()).collect(),
                    ships: ships.clone(),
                    controlled_by: controlled_by.clone(),
                    connects_to: connects_to.clone() 
                }
            },
        }
    }
}

impl System{
    pub fn has_presence(self: &System, player_color: &Color) -> bool {
        match self {
            System::Unused => false,
            System::Used {ships, ..} => {
                let s =ships.get(player_color).unwrap();
                s.damaged >= 1 || s.fresh >= 1
            }
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
                let (new_controlled_by, _) = ships.iter().fold((None, 0), |acc: (Option<Color>, u8), (c,s)| {
                    match acc {
                        (None, max_count) => if s.fresh > max_count {(Some(c.clone()), s.fresh)} else {(None, max_count)},
                        (Some(max_player), max_count) => {
                            if s.fresh > max_count {
                                (Some(c.clone()), s.fresh)
                            } else if s.fresh == max_count && c != &max_player && s.fresh != 0 {
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
                ships.get(color).unwrap().fresh
            }
        }
    }

    pub fn get_all_ships(self:&System, color: &Color) -> u8 {
        match self {
            System::Unused => 0,
            System::Used { ships, .. } => {
                let s = ships.get(color).unwrap();
                s.fresh + s.damaged
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cluster {
    gate_id: u8,
    systems_id: Vec<u8>
}