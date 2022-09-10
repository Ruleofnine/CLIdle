use crate::{
    app::App,
    player::Player,
    upgrades::Upgrade,
    utils::{format_number, if_can_buy_style},
};
use rug::{float::Round, Float};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap,TableState,ListState},
    Frame,
};
fn calc_prestige_points_gain(player: &Player) -> Float {
    //placeholder formula
    let prestige_divided = &player.points / Float::with_val(50, 1e5);
    let prestige_points = Float::with_val_round(50, prestige_divided.log2_ref(), Round::Down);
    if prestige_points.0 > 0 {
        prestige_points.0
    } else {
        Float::new(10)
    }
}

pub fn draw_prestige_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(40), Constraint::Min(0)].as_ref())
        .split(area);
    let point_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(6),Constraint::Length(3),Constraint::Min(6)].as_ref())
        .split(horizontal_chunks[0]);
    draw_prestige_text(f,point_chunks[0],&app.player); 
    draw_prestige_button(f,point_chunks[1],app)
   }
fn draw_prestige_button<B>(f: &mut Frame<B>, area: Rect,app:&mut App)
where
    B: Backend,
{
    let text = vec![
        Spans::from(vec![
            Span::styled("              PRESTIGE?",Style::default().add_modifier(Modifier::BOLD))
                  ])
    ];
    let block = Block::default().borders(Borders::ALL).title(Span::styled(
        "",
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD),
    ));
    // let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    let prestige_button: Vec<ListItem> = vec![ListItem::new(text)];
    f.render_stateful_widget(
        List::new(prestige_button).block(block).highlight_symbol(">>>").highlight_style(Style::default().fg(Color::Red).bg(Color::Black)),
        area,
        &mut app.prestige_state.state,
    );
}
fn draw_prestige_text<B>(f: &mut Frame<B>, area: Rect, player: &Player)
where
    B: Backend,
{
    let text = vec![
        Spans::from(vec![
            Span::from("Prestige Points: "),
            Span::styled(
                format_number(&player.prestige_points),
                Style::default().add_modifier(Modifier::BOLD),
            ),
        ]),
        Spans::from(vec![
            Span::from("Points on Prestige: "),
            Span::styled(
                format_number(&calc_prestige_points_gain(&player)),
                Style::default().add_modifier(Modifier::BOLD),
            ),
        ]),
    ];
    let block = Block::default().borders(Borders::ALL).title(Span::styled(
        "Prestige",
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD),
    ));
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}
