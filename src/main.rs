use std::{io::{self, Stdout}, thread, time::Duration, env::{args, Args}};
use tui::{
    backend::CrosstermBackend,
    widgets::{Widget, Block, Borders},
    layout::{Layout, Constraint, Direction, Rect},
    Terminal, Frame
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

struct PomorustoArgs {
    duration: u64,
}

fn main() -> Result<(), io::Error> {
    let args: Args = args();    
    let pomorusto_args: PomorustoArgs = parse_arguments(args);

    let mut stdout: Stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend: CrosstermBackend<Stdout> = CrosstermBackend::new(stdout);
    let mut terminal: Terminal<CrosstermBackend<Stdout>> = Terminal::new(backend)?;

    terminal.draw(|frame: &mut Frame<CrosstermBackend<Stdout>>| {
        let size: Rect = frame.size();
        let block: Block = Block::default().title("Rustodoro").borders(Borders::ALL);
        frame.render_widget(block, size);
    })?;

    thread::sleep(Duration::from_millis(pomorusto_args.duration));

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    return Ok(());
}

fn parse_arguments(mut args: Args) -> PomorustoArgs {
    let duration_string: String = args.nth(1).unwrap();

    let duration: u64 = duration_string.parse::<u64>().unwrap();

    return PomorustoArgs { duration };
}
