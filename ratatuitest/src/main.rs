use crossterm::event::KeyCode;
use crossterm::event::KeyEventKind;
use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::enable_raw_mode;
use ratatui::prelude::Backend;
use ratatui::prelude::CrosstermBackend;
use ratatui::prelude::Rect;
use ratatui::style::Modifier;
use ratatui::style::Style;
use ratatui::widgets::List;
use ratatui::widgets::ListItem;
use ratatui::widgets::ListState;
use ratatui::Frame;
use ratatui::Terminal;
use std::error::Error;

struct Selector {
    pub state: ListState,
    pub values: Vec<String>,
    pub selected: Vec<usize>,
}

impl Selector {
    pub fn new(values: Vec<String>) -> Self {
        Self {
            state: ListState::default().with_selected(Some(0)),
            values,
            selected: Vec::new(),
        }
    }

    pub fn down(&mut self) {
        let current_selected = self.selected();
        if current_selected >= self.values.len() - 1 {
            self.state.select(Some(0))
        } else {
            self.state.select(Some(current_selected + 1));
        }
    }

    pub fn up(&mut self) {
        let current_selected = self.selected();
        if current_selected <= 0 {
            self.state.select(Some(self.values.len() - 1));
        } else {
            self.state.select(Some(current_selected - 1))
        }
    }

    fn selected(&self) -> usize {
        match self.state.selected() {
            Some(i) => i,
            None => 0,
        }
    }

    fn mark(&mut self) {
        let current_selected = self.selected();
        match self
            .selected
            .iter()
            .position(|&value| value == current_selected)
        {
            Some(x) => _ = self.selected.remove(x),
            None => self.selected.push(current_selected),
        };
    }

    fn marked(&self) -> Vec<&str> {
        self.selected
            .iter()
            .map(|p| self.values[*p].as_str())
            .collect()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let stdout = std::io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let terminal_rect = terminal.size().unwrap();

    println!("---");
    // since we need 5 lines...
    for _ in 0..5 {
        println!("");
    }
    let cursor_pos = terminal.get_cursor().unwrap();

    let mut options = Selector::new(vec![
        "Option 1".into(),
        "Option 2".into(),
        "Option 3".into(),
        "Option 4".into(),
        "Option 5".into(),
        "Option 6".into(),
    ]);
    let lower_rect = Rect::new(0, cursor_pos.1 - 5, terminal_rect.width, 5);
    run_app(&mut terminal, &lower_rect, &mut options)?;

    terminal.set_cursor(0, cursor_pos.1)?;

    // finish
    disable_raw_mode()?;

    println!("Selected: {:?}", options.marked());
    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    pos: &Rect,
    values: &mut Selector,
) -> std::io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, pos, values))?;

        match crossterm::event::read()? {
            crossterm::event::Event::Key(key) => {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Esc => break,
                        KeyCode::Enter => break,
                        KeyCode::Down => values.down(),
                        KeyCode::Up => values.up(),
                        KeyCode::Char(' ') => values.mark(),
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }
    Ok(())
}

fn ui<B: Backend>(f: &mut Frame<B>, pos: &Rect, cursor: &mut Selector) {
    let items = cursor
        .values
        .iter()
        .enumerate()
        .map(|(pos, desc)| {
            ListItem::new(format!(
                "{} {}",
                if cursor.selected.contains(&pos) {
                    "[x]"
                } else {
                    "[ ]"
                },
                desc.to_string()
            ))
        })
        .collect::<Vec<ListItem>>();
    let list = List::new(items)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");
    f.render_stateful_widget(list, *pos, &mut cursor.state);
}
