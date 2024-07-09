use super::app::Message;
use std::{sync::mpsc::Sender, thread, time::Duration};

pub fn run_loop(bus: Sender<Message>) -> ! {
    loop {
        let msg = Message::SamusMoved { x: 0, y: 0 };

        bus.send(msg).unwrap();
        thread::sleep(Duration::from_millis(100))
    }
}
