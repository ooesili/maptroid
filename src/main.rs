use anyhow::{anyhow, Context, Result};
use maptroid::{
    app::{GameData, Message, Model, Position, SideEffect},
    gamedata::{load_all_rooms, load_room_diagrms},
    snes,
    tui::{self, view, Tui},
};
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::{env, fs::File, sync::mpsc, thread, time::Duration};
use tracing::info;

fn main() -> Result<()> {
    let file = File::create("maptroid.log").context("opening log file")?;
    tracing_subscriber::fmt()
        .with_writer(file)
        .with_ansi(false)
        .init();

    let mut tui = tui::init().context("initializing terminal")?;
    tui.clear()?;

    info!("starting main loop");
    let result = main_loop(tui);

    tui::restore().context("restoring terminal")?;
    result
}

fn main_loop(mut tui: Tui) -> Result<()> {
    let mut model = init_model().context("initializing")?;
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

fn init_model() -> Result<Model> {
    let sm_json_data_path =
        env::var_os("SM_JSON_DATA").ok_or_else(|| anyhow!("$SM_JSON_DATA not set"))?;
    let region_diagrams = load_room_diagrms(&sm_json_data_path)?;
    let rooms = load_all_rooms(&sm_json_data_path)?;

    Ok(Model {
        samus: Position::default(),
        game_data: GameData {
            rooms,
            region_diagrams,
        },
    })
}

fn run_ui_loop(bus: mpsc::Sender<Message>) -> Result<()> {
    loop {
        let msg = read_next_event()
            .context("reading next event")?
            .and_then(event_to_message);
        if let Some(msg) = msg {
            info!(msg = ?msg, "sending message");
            bus.send(msg).unwrap();
        }
    }
}

fn read_next_event() -> Result<Option<Event>> {
    if event::poll(Duration::from_secs(1)).context("polling for event")? {
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
