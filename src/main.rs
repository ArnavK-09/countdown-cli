// Imports use color_eyre::Result;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    layout::Rect,
    style::{palette::tailwind, Color, Modifier, Style},
    text::Span,
    widgets::{Gauge, Widget},
    DefaultTerminal,
};
use std::env;
use std::time::{Duration, Instant};

// Constants
const BACKGROUND_COLOR: Color = tailwind::GRAY.c800;
const PROGRESS_COLOR: Color = tailwind::GRAY.c950;
const DEFAULT_COUNTDOWN_TIME_IN_MS: &str = "60s";

// Base CountdownApp traits
#[derive(Debug, Default, Clone, Copy)]
struct CountdownApp {
    over: bool,
    elapsed_time_in_ms: u64,
    total_time_in_ms: u64,
}

// Utilities
fn human_readable_to_milliseconds(input: &str) -> Option<u64> {
    let mut milliseconds = 0u64;
    for part in input.split_whitespace() {
        if let Ok(num) = part[..part.len() - 1].parse::<u64>() {
            match part.chars().last()? {
                'h' => milliseconds += num * 60 * 60 * 1000, // Hours
                'm' => milliseconds += num * 60 * 1000,      // Minutes
                's' => milliseconds += num * 1000,           // Seconds
                _ => return None,                            // Invalid format
            }
        } else {
            return None;
        }
    }
    Some(milliseconds)
}

fn milliseconds_to_human_readable(ms: u64) -> String {
    let hours = ms / (60 * 60 * 1000);
    let minutes = (ms % (60 * 60 * 1000)) / (60 * 1000);
    let seconds = (ms % (60 * 1000)) / 1000;
    format!("{:02}h {:02}m {:02}s", hours, minutes, seconds)
}

// Entrypoint
fn main() -> Result<(), color_eyre::Report> {
    color_eyre::install()?;
    let app = CountdownApp::default().run(ratatui::init());
    ratatui::restore();
    app
}

// App methods
impl CountdownApp {
    fn run(mut self, mut terminal: DefaultTerminal) -> Result<(), color_eyre::Report> {
        let args: Vec<String> = env::args().collect();
        self.total_time_in_ms =
            human_readable_to_milliseconds(DEFAULT_COUNTDOWN_TIME_IN_MS).unwrap_or(60_000);
        if let Some(duration) = args
            .get(1)
            .and_then(|arg| human_readable_to_milliseconds(arg))
        {
            self.total_time_in_ms = duration;
        }

        let start_time = Instant::now();

        while !self.over {
            self.elapsed_time_in_ms = start_time.elapsed().as_millis() as u64;

            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn handle_events(&mut self) -> Result<(), color_eyre::Report> {
        let timeout = Duration::from_secs_f32(1.0 / 20.0);
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Esc => self.over = true,
                        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            self.over = true
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }
}

// GUI Implementation
impl Widget for &CountdownApp {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let ratio = self.elapsed_time_in_ms as f64 / self.total_time_in_ms as f64;
        let ratio = ratio.clamp(0.0, 1.0);
        let remaining_time = if self.elapsed_time_in_ms >= self.total_time_in_ms {
            "Time's Up!".to_string()
        } else {
            milliseconds_to_human_readable(self.total_time_in_ms - self.elapsed_time_in_ms)
        };
        Gauge::default()
            .gauge_style(
                Style::default()
                    .fg(PROGRESS_COLOR)
                    .bg(BACKGROUND_COLOR)
                    .add_modifier(Modifier::BOLD),
            )
            .ratio(ratio)
            .label(Span::styled(
                remaining_time,
                Style::default()
                    .fg(tailwind::WHITE)
                    .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
            ))
            .render(area, buf);
    }
}
