use crossterm::event::KeyCode;
use crossterm::event::KeyEventKind;
use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::enable_raw_mode;
use ratatui::prelude::Backend;
use ratatui::prelude::Constraint;
use ratatui::prelude::CrosstermBackend;
use ratatui::prelude::Direction;
use ratatui::prelude::Layout;
use ratatui::style::Color;
use ratatui::style::Modifier;
use ratatui::style::Style;
use ratatui::widgets::List;
use ratatui::widgets::ListItem;
use ratatui::Frame;
use ratatui::Terminal;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let res = run_app(&mut terminal);

    // finish
    disable_raw_mode()?;
    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> std::io::Result<()> {
    loop {
        terminal.draw(|f| ui(f))?;

        match crossterm::event::read()? {
            crossterm::event::Event::Key(key) => {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Esc => break,
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }
    Ok(())
}

fn ui<B: Backend>(f: &mut Frame<B>) {
    let items = [
        ListItem::new("Item 1"),
        ListItem::new("Item 2"),
        ListItem::new("Item 3"),
    ];
    let block = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());

    let list = List::new(items)
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");
    f.render_widget(list, block[0]);
}
