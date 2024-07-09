pub struct Model {}

impl Model {
    pub fn new() -> Model {
        Model {}
    }

    pub fn update(&mut self, msg: Message) -> SideEffect {
        match msg {
            Message::SamusMoved { x, y } => {
                // TODO
                SideEffect::Continue
            }
            Message::Quit => SideEffect::Stop,
            Message::Resize { .. } => SideEffect::Continue,
        }
    }
}

pub struct Point(u8, u8);

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
