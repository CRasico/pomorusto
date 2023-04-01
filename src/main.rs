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
    time::{Duration, self},
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Gauge, Widget, List, ListItem},
    Frame, Terminal, style::{Style, Color, Modifier},
};

struct PomorustoArgs {
    duration: u64,
    tasks: Vec<String>
}

fn main() -> Result<(), io::Error> {
    let args: Args = args();
    let pomorusto_args: PomorustoArgs = parse_arguments(args);

    let mut terminal: Terminal<CrosstermBackend<Stdout>> = create_terminal()?;

    let duration: Duration = Duration::from_secs(pomorusto_args.duration);
    let timer = time::Instant::now();

    loop {
        let elapsed: Duration= timer.elapsed();
        if duration < elapsed {
            break;
        }

        let percent_complete: f32 = (elapsed.as_secs_f32() / duration.as_secs_f32()) * 100 as f32 ;
        draw(&mut terminal, percent_complete as u16, pomorusto_args.tasks.clone())?;
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    return Ok(());
}

fn create_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, io::Error> {
    let mut stdout: Stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend: CrosstermBackend<Stdout> = CrosstermBackend::new(stdout);
    let mut terminal: Terminal<CrosstermBackend<Stdout>> = Terminal::new(backend)?;

    return Ok(terminal);
}

fn draw(terminal: &mut Terminal<CrosstermBackend<Stdout>>, percentage: u16, tasks: Vec<String>) -> Result<(), io::Error>{
    terminal.draw(|frame: &mut Frame<CrosstermBackend<Stdout>>| {
        let items: Vec<ListItem> =  tasks.into_iter().map(|task: String| {
            return ListItem::new(task);
        }).collect();

        let chunks: Vec<Rect> = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(30),
                Constraint::Percentage(70),
            ].as_ref()
        )
        .split(frame.size());

        let gauge: Gauge = Gauge::default().block(
            Block::default()
                .title("Time Remaining")
                .borders(Borders::ALL),
        ).gauge_style(Style::default().fg(Color::Cyan).bg(Color::Black).add_modifier(Modifier::ITALIC)).percent(percentage);

        let items: List = List::new(items)
            .block(Block::default().title("Tasks").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>");

        frame.render_widget(gauge, chunks[0]);
        frame.render_widget(items, chunks[1]);
    })?;

    return Ok(());
}

fn parse_arguments(mut args: Args) -> PomorustoArgs {
    let duration_string: String = args.nth(1).unwrap();

    let duration: u64 = duration_string.parse::<u64>().unwrap();

    let mut list_items: Vec<String> = vec![];
    let mut next = args.next();
    while next.is_some() {
        let action_item: String = next.unwrap();
        list_items.push(action_item);
        next = args.next();
    }

    return PomorustoArgs { duration, tasks: list_items};
}
