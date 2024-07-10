use super::app::Model;
use anyhow::Result;
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    layout::{Alignment, Rect},
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph},
    Terminal,
};
use std::io::{self, stdout, Stdout};
/// A type alias for the terminal type used in this application
pub type Tui = Terminal<CrosstermBackend<Stdout>>;

/// Initialize the terminal
pub fn init() -> io::Result<Tui> {
    execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

/// Restore the terminal to its original state
pub fn restore() -> io::Result<()> {
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

pub fn view(tui: &mut Tui, model: &Model) -> Result<()> {
    tui.draw(|frame| {
        let frame_area = frame.size();

        let block = Block::bordered()
            .title("maptroid")
            .title_alignment(Alignment::Center);
        let map_area = Rect {
            x: frame_area.x + 1,
            y: frame_area.y + 1,
            width: frame_area.width - 2,
            height: frame_area.height - 2,
        };

        let lines = (0..map_area.height)
            .map(|_| Line::from((0..map_area.width).map(|_| ".".blue()).collect::<Vec<_>>()))
            .collect::<Vec<Line>>();

        frame.render_widget(Paragraph::new(lines).block(block), frame_area);
    })?;
    Ok(())
}
