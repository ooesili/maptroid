#[derive(Debug, Default)]
pub struct Model {
    pub samus: Position,
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
