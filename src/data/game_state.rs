use std::clone;
use std::collections::HashMap;

use actix_web::Resource;

use crate::data::game_state;

use super::court_cards::{CourtCard, VoxPayload, Guild};
use super::system::{System};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Color {
    Red,
    Blue,
    White,
    Yellow
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResourceType {
    Fuel,
    Material,
    Weapons,
    Relics,
    Psionics
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PreludeActionPayload {
    Interest {target_resource: ResourceType, steal_from: Vec<(Color,u8)>},
    Steal {target_resource: ResourceType, target_player: ResourceType},
    Union {card_type: ActionType, },
    PlaceShips {target_system: u8},
    Farseers {cards: Vec<ActionCard>},
    RelicFence {target_resource: ResourceType},
    SilverTounges,
    ElderBroker,
    GateKeepers
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ResourceSlot {
    Used {keys: u8, resource: ResourceType},
    Unused {keys: u8},
    Covered {keys: u8}
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Agents{
    pub color: Color,
    pub count: u8
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Trophy {
    pub trophy_type: ReserveType,
    pub count: u8,
    pub player: Color
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BuildType{
    Starport,
    City,
    Ship
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Dice{
    Skirmish,
    Assault,
    Raid
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BasicAction{
    Build {target_system: u8, build_type: BuildType},
    Repair {target_system: u8, build_type: BuildType},
    Tax {target_system: u8, target_player: Color},
    Influence {card_id: u8},
    Move {origin_id: u8, destination_id: u8, fresh_ships: u8, damaged_ships: u8},
    Secure {card_id: u8, vox_payload: Option<VoxPayload>},
    Battle {target_system: u8, target_player: Color, dice: Vec<Dice>},
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Action{
    PlayLeadCard {card: ActionCard, declare: Option<AmbitionTypes>},
    Pass,
    Surpass {card: ActionCard, seize: Option<ActionCard>},
    Copy {card: ActionCard, seize: Option<ActionCard>},
    Pivot {card: ActionCard, seize: Option<ActionCard>},
    PreludeResourceAction {basic_action: BasicAction, used_resource: u8},
    UseWeapons {used_resource: u8},
    PreludeCard {guild_card: u8, prelude_action_payload: PreludeActionPayload},
    EndPrelude,
    MainAction {basic_action: BasicAction},
    AllocateResources {configuration: Vec<(u8,ResourceType)>},
    EndTurn
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ActionType{
    Administration,
    Agression,
    Construction,
    Mobilization
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionCard{
    pub action_type: ActionType,
    pub number: u8,
    pub pips: u8,
    pub declared_ambition: Option<AmbitionTypes>
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ReserveType {
    Ships,
    Agents,
    Starports,
    Cities
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlayerArea {
    pub player: Color,
    pub power: u8,
    pub initiative: bool,
    pub action_cards: Vec<ActionCard>,
    pub guild_cards: Vec<Guild>,
    pub reserve: HashMap<ReserveType,u8>,
    pub resource_slots: Vec<ResourceSlot>,
    pub captives: Vec<Agents>,
    pub tropies: Vec<Trophy>
}

impl PlayerArea {
    pub fn add_trophies(&self, tropies: Vec<Trophy>) -> Vec<Trophy>{
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
        return combined;
    }

    pub fn add_action_cards(&mut self, cards: Vec<ActionCard>) {
        self.action_cards.extend(cards);
    }

    pub fn remove_action_card(&mut self, card: ActionCard) {
        let ind = self.action_cards.iter().position(|c| *c == card);
        match ind {
            Some(i) => { self.action_cards.remove(i); }
            None => panic!("Action Card does not exist: {:?} in {:?}", card, self.player),
        }
    }

    pub fn change_reserve(&self, reserve_type: &ReserveType, diff: i8) -> PlayerArea {
        let new_reserve = self.reserve.iter().map(|(k, v)| {
            if k == reserve_type {
                let new_value = (*v as i8) + diff;
                if new_value >= 0 {(k.clone(), new_value as u8)} else {panic!("Cannot take {:?} reserves from {:?} {:?}", -diff, v, k)}
            }
            else {
            (k.clone(), v.clone())
                }
        }).collect();
        PlayerArea { reserve: new_reserve, .. self.clone()}
    }

    pub fn remove_resource(&self, resource_slot: u8, target_resource: &ResourceType) -> PlayerArea {
        let new_resource_slots: Vec<ResourceSlot>= self.resource_slots.iter().enumerate().map(|(i, r)| {
            if i == resource_slot as usize {
            match r {
                ResourceSlot::Used { keys, resource } if resource == target_resource => ResourceSlot::Unused { keys: *keys },
                _ => panic!("Cannot remove {:?} from {:?}", target_resource, r)
            }} else {
                r.clone()
            }
        }).collect();
        PlayerArea {
            resource_slots: new_resource_slots,
            .. self.clone()
        }
    }

    pub fn get_resource(&self, resource_slot: u8) -> Option<ResourceType> {
        match &self.resource_slots[resource_slot as usize] {
            ResourceSlot::Used { resource, .. } => Some(resource.clone()),
            _ => None,
        }
    }

    pub fn update_resource_slots(&self) -> (PlayerArea, Vec<ResourceType>) {
        let blocked_slots = match self.reserve.get(&ReserveType::Cities).unwrap() {
            5 => [false, false, true, true, true, true],
            4 => [false, false, false, true, true, true],
            3 => [false, false, false, false, true, true],
            _ => [false, false, false, false, false, false],
        };
        let (new_resource_slots, overflow_resources): (Vec<_>, Vec<_>) = self.resource_slots.iter().zip(blocked_slots).map(|(slot, blocked)| {
            match (slot, blocked) {
                (ResourceSlot::Used { keys, resource }, true)                          => (ResourceSlot::Covered { keys: *keys }, Some(resource.clone())),
                (ResourceSlot::Covered { keys }, false) | (ResourceSlot::Unused { keys }, false) => (ResourceSlot::Unused { keys: *keys }, None),
                (ResourceSlot::Covered { keys }, true)  | (ResourceSlot::Unused { keys }, true)  => (ResourceSlot::Covered { keys: *keys }, None),
                (ResourceSlot::Used { keys, resource }, false)                         => (ResourceSlot::Used { keys: *keys, resource: resource.clone() }, None)
            }
        }).unzip();
        (PlayerArea { resource_slots: new_resource_slots, ..self.clone() }, overflow_resources.into_iter().filter_map(|o| o).collect())
    }

    pub fn get_resource_count(&self, resource: ResourceType) -> u8 {
        let guild_resources = self.guild_cards.iter().filter(|g| g.resource == resource).count();
        let resources = self.resource_slots.iter().filter(|s| match s {
            ResourceSlot::Used { keys: _, resource: _resource } => true,
            _ => false
        }).count();
        (guild_resources+resources) as u8
    }

    pub fn get_trophies(&self) -> u8 {
        self.tropies.iter().map(|t| t.count).sum()
    }

    pub fn get_captives(&self) -> u8 {
        self.captives.iter().map(|t| t.count).sum()
    }

}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AmbitionMarker{
    pub first_place: u8,
    pub second_place: u8,
    pub flipped: bool,
    pub first_place_flipped: u8,
    pub second_place_flipped: u8
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum AmbitionTypes {
    Tycoon,
    Tyrant,
    Warlord,
    Keeper,
    Empath
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ambition{
    pub ambition_type: AmbitionTypes,
    pub markers: Vec<AmbitionMarker>,
    pub discarded_resources: Vec<ResourceType>
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TurnState {
    TrickTaking,
    Prelude {action_type: ActionType, pips_left: u8},
    Actions {action_type: ActionType, pips_left: u8},
    AllocateResources {player: Color, resources: Vec<ResourceType>},
    AllocateDiceResults {target_system: u8, target_player: Color, self_hits: u8, hits: u8, building_hits: u8, keys: u8}
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameState {
    pub players: HashMap<Color, PlayerArea>,
    pub current_player: Color,
    pub players_in_round: u8,
    pub initiative: Color,
    pub seized: Option<Color>,
    pub zero_marker: bool,
    pub turn_state: TurnState,
    pub next_turn_states: Vec<TurnState>,
    pub chapter: u8,
    pub systems: Vec<System>,
    pub resource_reserve: HashMap<ResourceType, u8>,
    pub court: Vec<CourtCard>,
    pub court_draw_pile: Vec<CourtCard>,
    pub court_discard_pile: Vec<CourtCard>,
    pub action_discard: Vec<ActionCard>,
    pub lead_card: Option<(ActionCard, bool, Color)>,
    pub follow_cards: Vec<(ActionCard, bool, Color)>,
    pub ambition_markers: Vec<AmbitionMarker>,
    pub ambitions: HashMap<AmbitionTypes, Ambition>
}

impl GameState {
    pub fn get_player_area(&self, color: &Color) -> PlayerArea {
        self.players.get(color).expect("Cannot find PlayerArea").clone()
    }

    pub fn add_action_cards(&mut self, color: &Color, cards: Vec<ActionCard>) {
        let mut player_area = self.get_player_area(color);
        player_area.add_action_cards(cards);
        self.players.insert(color.clone(), player_area);
    }

    pub fn update_players_reserve(&self, player: &Color, reserve_type: &ReserveType, diff: i8) -> GameState {
        let new_players = self.players.iter().map(|(color, area)| {
            if color == player {
                (color.clone(), area.change_reserve(reserve_type, diff))
            } else {
                (color.clone(), area.clone())
            }
        }).collect();

        GameState {
            players: new_players,
            .. self.clone()
        }
    }

    pub fn redraw_court_cards(&self) -> GameState {
        let max_court_cards = if self.players.len() == 2 {3} else {4};
        let new_court = if self.court.len() < max_court_cards {
            self.court.iter().chain(vec![&self.court_draw_pile[0]]).cloned().collect()
        } else {self.court.clone()};

        let new_draw_pile = if self.court.len() < max_court_cards {
            self.court_draw_pile.iter().skip(1).cloned().collect()
        } else {self.court_draw_pile.clone()};

        let new_game_state = GameState {
            court: new_court.clone(),
            court_draw_pile: new_draw_pile.clone(),
            .. self.clone()
        };

        if new_court.len() == max_court_cards || new_draw_pile.len() == 0 {return new_game_state;} else {return  new_game_state.redraw_court_cards();}
        
    }

    pub fn remove_resource(&self, player: &Color, resource_slot: u8, target_resource: ResourceType) -> GameState {
        let new_players = self.players.iter().map(|(color, area)| {
            if color == player {
                (color.clone(), area.remove_resource(resource_slot, &target_resource))
            } else {
                (color.clone(), area.clone())
            }
        }).collect();

        GameState {
            players: new_players,
            ..self.clone()
        }
    }

}