use anyhow::{Context, Result};
use maptroid::{
    app::{Message, Model, SideEffect},
    snes,
    tui::{self, Tui},
};
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    style::Stylize,
    widgets::Paragraph,
};
use std::{sync::mpsc, thread, time::Duration};

fn main() -> Result<()> {
    let mut tui = tui::init().context("initializing terminal")?;
    tui.clear()?;

    let result = main_loop(tui);

    tui::restore().context("restoring terminal")?;
    result
}

fn main_loop(mut tui: Tui) -> Result<()> {
    let mut model = Model::default();
    let (sender, receiver) = mpsc::channel();

    let snes_sender = sender.clone();
    let ui_sender = sender.clone();
    thread::spawn(move || snes::run_loop(snes_sender));
    thread::spawn(move || run_ui_loop(ui_sender).expect("TODO: error handling"));

    view(&mut tui, &model).context("drawing initial UI")?;
    loop {
        let msg = receiver.recv().unwrap();

        match model.update(msg) {
            SideEffect::Continue => {}
            SideEffect::Stop => return Ok(()),
        }

        view(&mut tui, &model).context("drawing UI")?;
    }
}

fn run_ui_loop(bus: mpsc::Sender<Message>) -> Result<()> {
    let message = read_next_event()
        .context("reading next event")?
        .and_then(event_to_message);
    if let Some(message) = message {
        bus.send(message).unwrap();
    }
    Ok(())
}

fn view(tui: &mut Tui, model: &Model) -> Result<()> {
    tui.draw(|frame| {
        let area = frame.size();
        frame.render_widget(
            Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                .white()
                .on_blue(),
            area,
        );
    })?;
    Ok(())
}

fn read_next_event() -> Result<Option<Event>> {
    if event::poll(Duration::from_millis(16)).context("polling for event")? {
        return Ok(Some(event::read().context("reading event")?));
    }
    Ok(None)
}

fn event_to_message(event: Event) -> Option<Message> {
    Some(match event {
        Event::Key(key) => {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                Message::Quit
            } else {
                return None;
            }
        }
        Event::Resize(cols, rows) => Message::Resize { cols, rows },
        _ => return None,
    })
}
