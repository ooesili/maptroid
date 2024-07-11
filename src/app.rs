use std::collections::HashMap;

use super::gamedata::{RegionDiagram, Room};

#[derive(Debug)]
pub struct Model {
    pub samus: Position,
    pub game_data: GameData,
}

#[derive(Debug)]
pub struct GameData {
    pub region_diagrams: HashMap<String, RegionDiagram>,
    pub rooms: Vec<Room>,
}

#[derive(Debug, Default)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

impl Model {
    pub fn update(&mut self, msg: Message) -> SideEffect {
        match msg {
            Message::SamusMoved { x, y } => {
                self.samus = Position { x, y };
                SideEffect::Continue
            }
            Message::Quit => SideEffect::Stop,
            Message::Resize { .. } => SideEffect::Continue,
        }
    }
}

#[derive(Debug)]
pub enum Message {
    Resize { cols: u16, rows: u16 },
    SamusMoved { x: u8, y: u8 },
    Quit,
}

#[derive(Debug)]
pub enum SideEffect {
    Continue,
    Stop,
}
