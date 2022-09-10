use crate::{
    app::{App, TabType},
    debug_menu,
    player::Player,
    upgrades::Upgrade,
    utils::format_number,
    tabs::{points::draw_first_tab,prestige::draw_prestige_tab},
};
use rug::Float;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Tabs, Wrap},
    Frame,
};

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if self.items.len() == 0 {
                    0
                } else if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if self.items.len() == 0 {
                    0
                } else if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
    pub fn insert_and_goto_bottom(&mut self, new_entry: T) {
        self.items.insert(self.items.len(), new_entry);
        self.state.select(Some(self.items.len() - 1));
    }
}
pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());
    let titles = app
        .tabs
        .tabs
        .iter()
        .map(|t| Spans::from(Span::styled(t.0, Style::default().fg(Color::Green))))
        .collect();

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(app.title))
        .highlight_style(Style::default().fg(Color::LightMagenta))
        .select(app.tabs.index);
    //Renders Tabs in the top chunk
    f.render_widget(tabs, chunks[0]);
    match app.tabs.tabtype {
        //Send the rest of the terminal chunk to tab to render
        TabType::Main => draw_first_tab(f, app, chunks[1]),
        TabType::Debug => debug_menu::draw_debug_tab(f, app, chunks[1]),
        TabType::Prestige => draw_prestige_tab(f, app, chunks[1]),
        // 2 => draw_third_tab(f, app, chunks[1]),
        _ => {}
    };
}



