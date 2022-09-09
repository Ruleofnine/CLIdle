use crate::{
    app::{App, TabType},
    debug_menu,
    player::Player,
    upgrades::Upgrade,
    utils::format_number,
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
fn draw_prestige_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(40), Constraint::Min(0)].as_ref())
        .split(area);
    //Split again the top 4 for the points
    let point_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(10)].as_ref())
        .split(horizontal_chunks[0]);
    draw_prestige_text(f, point_chunks[0], &app.player);
}
fn draw_prestige_text<B>(f: &mut Frame<B>, area: Rect, player: &Player)
where
    B: Backend,
{
    let text = vec![Spans::from(vec![
        Span::from("Prestige Points: "),
        Span::styled(
            format_number(&player.prestige_points),
            Style::default().add_modifier(Modifier::BOLD),
        ),
    ])];
    let block = Block::default().borders(Borders::ALL).title(Span::styled(
        "Prestige",
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD),
    ));
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}

fn draw_first_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
//Receive the second chunk
where
    B: Backend,
{
    //split 5 inward for points and keypress upgrades
    let initial_horizontal_chunks = Layout::default()
        .constraints([Constraint::Length(5), Constraint::Min(0)].as_ref())
        .split(area);
    //split the first chunk generated in to approxtimately in the middle for points/keypress
    //upgraeds
    let points_keypress_upgrades_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(60), Constraint::Min(0)].as_ref())
        .split(initial_horizontal_chunks[0]);
    // let mut keypress_upgrades_list: Vec<ListItem> = Vec::new();
    // for u in &app.player.unowned_upgrades {
    //     if Float::with_val(10, &app.player.points * 3000) > u.cost {
    //         let color = if_can_buy_style(&u.cost, &app.player.points);
    //             keypress_upgrades_list.push(ListItem::new(Span::styled(
    //                 format!("{} Cost: {}", u.name, format_number(&u.cost)),
    //                 Style::default().fg(color).add_modifier(Modifier::BOLD),
    //             )))
    //

    // }
    // }
    // let keypress_upgrades_wiget = List::new(keypress_upgrades_list)
    //     .block(
    //         Block::default().borders(Borders::ALL).title(Span::styled(
    //             "Keypress Upgrades",
    //             Style::default()
    //                 .fg(Color::Magenta)
    //                 .add_modifier(Modifier::BOLD),
    //         )),
    //     )
    //     .highlight_symbol(">")
    //     .highlight_style(Style::default().fg(Color::Magenta));
    //
    // f.render_stateful_widget(
    //     keypress_upgrades_wiget,
    //     points_keypress_upgrades_chunks[1],
    //     &mut app.upgrade_state.state,
    // );
    // //Split chunk horizontally 30 inward
    app.upgrade_state.upgrade_indexes = Vec::new();
    let generator_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(50), Constraint::Min(20)].as_ref())
        .split(initial_horizontal_chunks[1]);
    draw_points_text(f, points_keypress_upgrades_chunks[0], &app.player);
    let mut upgrades_list: Vec<ListItem> = Vec::new();
    let mut gen_list: Vec<ListItem> = Vec::new();
    let mut upgrade_index = 0;
    for upgrade in &app.upgrade_list[0] {
        if Upgrade::near_cost(&app.player.points, &upgrade.cost) {
            let color = if_can_buy_style(&upgrade.cost, &app.player.points);
            upgrades_list.push(ListItem::new(Span::styled(
                format!("{} Cost: {}", upgrade.name, format_number(&upgrade.cost)),
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            )));
        app.upgrade_state.upgrade_indexes.push((0, upgrade_index));
        upgrade_index += 1;
        }
    }
    let gen_iter = app
        .player
        .owned_pointgenerators
        .items
        .iter_mut()
        .enumerate();
    for (gi, g) in gen_iter {
        let color = if_can_buy_style(&g.cost, &app.player.points);
        let style = Style::default().add_modifier(Modifier::BOLD).fg(color);
        let span = Spans::from(vec![Span::styled(
            format!(
                "{} Cost: {} [{}]",
                &g.name,
                format_number(&g.cost),
                format_number(&g.amount)
            ),
            style,
        )]);
        gen_list.push(ListItem::new(span));
        if g.amount > 0 {
            let mut upgrade_index = 0;
            for upgrade in &app.upgrade_list[gi + 1] {
                if Upgrade::near_cost(&app.player.points, &upgrade.cost) {
                    let color = if_can_buy_style(&upgrade.cost, &app.player.points);
                    upgrades_list.push(ListItem::new(Span::styled(
                        format!("{} Cost: {}", upgrade.name, format_number(&upgrade.cost)),
                        Style::default().fg(color).add_modifier(Modifier::BOLD),
                    )));
                app.upgrade_state.upgrade_indexes.push((gi+1, upgrade_index));
                }
                upgrade_index+=1;
            }
        }
    }
    app.upgrade_state.max_index = upgrades_list.len();
    let generator_list = List::new(gen_list)
        .block(
            Block::default().borders(Borders::ALL).title(Span::styled(
                "Point Generators",
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Magenta),
            )),
        )
        .highlight_symbol(">")
        .highlight_style(Style::default().fg(Color::Magenta));
    f.render_stateful_widget(
        generator_list,
        generator_chunks[0],
        &mut app.player.owned_pointgenerators.state,
    );

    let upgrades_wiget = List::new(upgrades_list)
        .block(
            Block::default().borders(Borders::ALL).title(Span::styled(
                "Generator Upgrades",
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            )),
        )
        .highlight_symbol(">")
        .highlight_style(Style::default().fg(Color::Magenta));

    f.render_stateful_widget(
        upgrades_wiget,
        generator_chunks[1],
        &mut app.upgrade_state.state,
    );
}
fn draw_points_text<B>(f: &mut Frame<B>, area: Rect, player: &Player)
where
    B: Backend,
{
    let text = vec![
        Spans::from(vec![
            Span::from("Points: "),
            Span::styled(
                format_number(&player.points),
                Style::default().add_modifier(Modifier::BOLD),
            ),
        ]),
        Spans::from(vec![
            Span::from("Points per second: "),
            Span::styled(
                format_number(&player.pps),
                Style::default().add_modifier(Modifier::BOLD),
            ),
        ]),
        Spans::from(vec![
            Span::from("Points per Keypress: "),
            Span::styled(
                format_number(&player.ppc),
                Style::default().add_modifier(Modifier::BOLD),
            ),
        ]),
    ];
    let block = Block::default().borders(Borders::ALL).title(Span::styled(
        "Points",
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    ));
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}
fn if_can_buy_style(cost: &Float, points: &Float) -> Color {
    let color: Color;
    if points > cost {
        color = Color::White
    } else {
        color = Color::Red
    }
    color
}
