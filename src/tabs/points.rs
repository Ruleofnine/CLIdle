use crate::{
    app::App,
    player::Player,
    upgrades::Upgrade,
    utils::{format_number,if_can_buy_style}

};
use rug::Float;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

pub fn draw_first_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
//Receive the second chunk
where
    B: Backend,
{
    let pointgenerator_border_color: Color;
    let upgrades_border_color: Color;
    if app.player.owned_pointgenerators.state.selected().is_some() {
        pointgenerator_border_color = Color::LightMagenta;
        upgrades_border_color = Color::White;
    } else if app.upgrade_state.state.selected().is_some() {
        pointgenerator_border_color = Color::White;
        upgrades_border_color = Color::LightMagenta;
    } else {
        pointgenerator_border_color = Color::White;
        upgrades_border_color = Color::White;
    }
    //split 5 inward for points and keypress upgrades
    let initial_horizontal_chunks = Layout::default()
        .constraints([Constraint::Length(5), Constraint::Min(0)].as_ref())
        .split(area);
    //split the first chunk generated in to 50 inward in the middle for points/keypress
    let points_keypress_upgrades_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(50), Constraint::Min(0)].as_ref())
        .split(initial_horizontal_chunks[0]);
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
                    app.upgrade_state
                        .upgrade_indexes
                        .push((gi + 1, upgrade_index));
                }
                upgrade_index += 1;
            }
        }
    }
    app.upgrade_state.max_index = upgrades_list.len();
    let generator_list = List::new(gen_list)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(pointgenerator_border_color))
                .title(Span::styled(
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
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(upgrades_border_color))
                .title(Span::styled(
                    "Upgrades",
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
                format_number(&Float::with_val(10, &player.ppc * &player.ppcmod)),
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
