use crate::{app::App,input::InputMode};
use tui::{
    backend::Backend,
    layout::{Rect,Constraint,Layout,Direction},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn draw_debug_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
//Receive the second chunk
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(6),
                Constraint::Length(10),
            ]
            .as_ref(),
        )
        .split(area);
    let input = Paragraph::new(app.input.as_ref())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Green),
        })
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(input, chunks[0]);
    let output = Paragraph::new(app.output.as_ref())
        .style(Style::default())
        .block(Block::default().borders(Borders::ALL).title("Output"));
    f.render_widget(output, chunks[1]);
    let messages: Vec<ListItem> = app
        .debug_info
        .items
        .iter()
        .enumerate()
        .map(|(i,m)| {
            let content = vec![Spans::from(Span::raw(format!("{:?}: {:?}", i,m)))];
            ListItem::new(content)
        })
        .collect();
    let messages = List::new(messages)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("History"),
        )
        .highlight_symbol(">> ")
        .highlight_style(Style::default().fg(Color::Green));
    f.render_stateful_widget(messages, chunks[2], &mut app.debug_info.state)
}

