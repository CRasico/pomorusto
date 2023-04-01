use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{
        self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
};
use std::{
    env::{args, Args},
    error::Error,
    io::{self, Stdout},
    thread,
    time::Duration,
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Gauge, Widget},
    Frame, Terminal, style::{Style, Color, Modifier},
};

struct PomorustoArgs {
    duration: u64,
}

fn main() -> Result<(), io::Error> {
    let args: Args = args();
    let pomorusto_args: PomorustoArgs = parse_arguments(args);

    let mut terminal: Terminal<CrosstermBackend<Stdout>> = create_terminal()?;

    thread::sleep(Duration::from_millis(pomorusto_args.duration));

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    return Ok(());
}

// TODO: Make This Stateful: https://docs.rs/tui/latest/tui/widgets/trait.StatefulWidget.html
fn create_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, io::Error> {
    let mut stdout: Stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend: CrosstermBackend<Stdout> = CrosstermBackend::new(stdout);
    let mut terminal: Terminal<CrosstermBackend<Stdout>> = Terminal::new(backend)?;

    terminal.draw(|frame: &mut Frame<CrosstermBackend<Stdout>>| {
        let chunks: Vec<Rect> = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(80),
                Constraint::Percentage(20),
            ].as_ref()
        )
        .split(frame.size());
        let block: Block = Block::default().title("Rustodoro Tasks").borders(Borders::ALL);
        let gauge: Gauge = Gauge::default().block(
            Block::default()
                .title("Time Remaining")
                .borders(Borders::ALL),
        ).gauge_style(Style::default().fg(Color::Cyan).bg(Color::Black).add_modifier(Modifier::ITALIC)).percent(30);
        frame.render_widget(block, chunks[0]);
        frame.render_widget(gauge, chunks[1])
    })?;

    return Ok(terminal);
}

fn parse_arguments(mut args: Args) -> PomorustoArgs {
    let duration_string: String = args.nth(1).unwrap();

    let duration: u64 = duration_string.parse::<u64>().unwrap();

    return PomorustoArgs { duration };
}
