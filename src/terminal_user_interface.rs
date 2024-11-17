// Imports
use crossterm::{execute, terminal::*};
use ratatui::prelude::*;
use std::io::{self, stdout, Stdout};

/// Type helper
pub type TUI = Terminal<CrosstermBackend<Stdout>>;

/// Initialize the terminal
pub fn init_tui() -> io::Result<TUI> {
    execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

/// Restore the terminal to its original
pub fn restore_tui() -> io::Result<()> {
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
