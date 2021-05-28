#![allow(dead_code)]
mod util;

use crate::trace::Input;
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

    let mut i_start = 0;
    let mut n_traces = 4;

    loop {
        terminal.draw(|f| {
            let constraints = vec![Constraint::Ratio(1, n_traces as u32); n_traces];
            let size = f.size();

            for (itr, trace) in traces.iter().skip(i_start).enumerate() {
                if itr >= n_traces {
                    break;
                }
                let ymin = &trace.ymin();
                let ymax = &trace.ymax();
                let xmin = 0.0;
                let xmax = trace.xdata.len() as f64;
                let data = &trace.xydata();

                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(constraints.as_ref())
                    .split(size);

                let datasets = vec![Dataset::default()
                    .marker(symbols::Marker::Braille)
                    .style(Style::default().fg(Color::Yellow))
                    .graph_type(GraphType::Line)
                    .data(data)];
                let chart = Chart::new(datasets)
                    .block(
                        Block::default()
                            .title(Span::styled(
                                format!("{}", trace.nslc_id()),
                                Style::default()
                                    .fg(Color::Cyan)
                                    .add_modifier(Modifier::BOLD),
                            ))
                            .borders(Borders::NONE),
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
                f.render_widget(chart, chunks[itr]);
            }
        })?;

        match events.next()? {
            Event::Input(input) => {
                if input == Key::Char('q') {
                    break;
                } else if input == Key::Char('j') && i_start + n_traces < traces.len() {
                    i_start += 1;
                } else if input == Key::Char('k') && i_start > 0 {
                    i_start -= 1;
                } else if input == Key::Char('+') {
                    n_traces += 1;
                } else if input == Key::Char('-') {
                    n_traces -= 1;
                }
            }
            Event::Tick => {}
        }
    }

    Ok(())
}
