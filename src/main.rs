// Import required
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};
use std::io::{self};

// Custom
mod terminal_user_interface;

// Our app class
#[derive(Debug, Default)]
pub struct TuiApp {
    count: i8,
    quit: bool,
}

// Appending functions required
impl TuiApp {
    fn run(&mut self, tui: &mut terminal_user_interface::TUI) -> io::Result<()> {
        while !self.quit {
            let _ = tui.draw(|f| self.render_this_frame(f));
            self.handle_all_events()?;
        }
        Ok(())
    }

    // Basic Functionalities

    fn render_this_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn handle_all_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_event_for_key(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_event_for_key(&mut self, e: KeyEvent) {
        match e.code {
            KeyCode::Char('q') => self.quit(),
            KeyCode::Left => self.decrement(),
            KeyCode::Right => self.increment(),
            _ => {}
        }
    }

    // Input Commands

    fn quit(&mut self) {
        self.quit = true;
    }

    fn increment(&mut self) {
        self.count += 1;
    }

    fn decrement(&mut self) {
        self.count -= 1;
    }
}

// Widget for demo app
impl Widget for &TuiApp {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Counter App Demo Corss-platform CLI ".bold());
        let instructions = Title::from(Line::from(vec![
            " Decrement ".bold().blue(),
            "<Left>".green().bold(),
            " Increment ".bold().blue(),
            "<Right>".green().bold(),
            " Quit ".bold().blue(),
            "<q> ".green().bold(),
        ]));
        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .borders(Borders::ALL)
            .border_set(border::THICK);

        let counter_render = Text::from(vec![Line::from(vec![
            Span::styled("    ", Style::new()),
            Span::styled(
                "  Your Counter Value:-  ",
                Style::new().fg(Color::White).bg(Color::Black).bold(),
            ),
            Span::styled("    ", Style::new()),
            self.count.to_string().yellow().bold(),
        ])]);

        Paragraph::new(counter_render)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

// Intializing App
fn main() -> io::Result<()> {
    // New tui
    let mut terminal = terminal_user_interface::init_tui()?;
    let terminal_app = TuiApp::default().run(&mut terminal);
    terminal_user_interface::restore_tui()?;
    terminal_app
}
