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

pub fn view(tui: &mut Tui, _model: &Model) -> Result<()> {
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

        let mut grid: Vec<Line> = vec![];
        for _y in 0..map_area.height {
            let mut row = vec![];
            for _x in 0..map_area.width {
                row.push(".".green());
            }
            grid.push(row.into());
        }
        frame.render_widget(Paragraph::new(grid).block(block), frame_area);
    })?;
    Ok(())
}
