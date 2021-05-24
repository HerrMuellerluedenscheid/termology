#![allow(dead_code)]
mod util;

use crate::trace::{Trace, Input};
use std::{error::Error, io};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    symbols,
    text::Span,
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType},
    Terminal,
};
use util::event::{Event, Events};

pub(crate) fn start(input: Input) -> Result<(), Box<dyn Error>> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();

    let traces = input.traces();
    let x = &traces[0];

    let trace = &traces[0];
    let ymin = &trace.ymin();
    let ymax = &trace.ymax();
    let xmin = 0.0;
    let xmax = trace.xdata.len() as f64;
    let data = &trace.xydata();

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Ratio(1, 3)].as_ref())
                .split(size);

            let datasets = vec![Dataset::default()
                .name("data")
                .marker(symbols::Marker::Braille)
                .style(Style::default().fg(Color::Yellow))
                .graph_type(GraphType::Line)
                .data(data)];
            let chart = Chart::new(datasets)
                .block(
                    Block::default()
                        .title(Span::styled(
                            "Chart 2",
                            Style::default()
                                .fg(Color::Cyan)
                                .add_modifier(Modifier::BOLD),
                        ))
                        .borders(Borders::ALL),
                )
                .x_axis(
                    Axis::default()
                        .title("x")
                        .style(Style::default().fg(Color::Gray))
                        .bounds([xmin, xmax])
                        .labels(vec![
                            Span::styled(
                                format!("{}", &trace.tmin()),
                                Style::default().add_modifier(Modifier::BOLD),
                            ),
                            Span::styled(
                                format!("{}", &trace.tmax()),
                                Style::default().add_modifier(Modifier::BOLD),
                            ),
                        ]),
                )
                .y_axis(
                    Axis::default()
                        .title("A")
                        .style(Style::default().fg(Color::Gray))
                        .bounds([*ymin, *ymax])
                        .labels(vec![
                            Span::styled(
                                format!("{}", ymin),
                                Style::default().add_modifier(Modifier::BOLD),
                            ),
                            Span::raw(format!("{}", ymin + (ymax - ymin) / 2.)),
                            Span::styled(
                                format!("{}", ymax),
                                Style::default().add_modifier(Modifier::BOLD),
                            ),
                        ]),
                );
            f.render_widget(chart, chunks[0]);
        })?;

        match events.next()? {
            Event::Input(input) => {
                if input == Key::Char('q') {
                    break;
                }
            }
            Event::Tick => {}
        }
    }

    Ok(())
}
