#![allow(dead_code)]
mod util;

use libmseed::MSTraceList;
use std::cmp::Ordering::Equal;
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

pub(crate) fn start(trace_list: MSTraceList) -> Result<(), Box<dyn Error>> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();

    let mut i_start = 0;
    let mut n_traces = 5;

    loop {
        terminal.draw(|f| {
            let traces = trace_list.traces();
            let constraints = vec![Constraint::Ratio(1, n_traces as u32); n_traces];
            let size = f.size();

            for (itr, tr) in traces.skip(i_start).enumerate() {
                if itr >= n_traces {
                    continue;
                }

                let trace = tr.segments().next().unwrap();
                let n_samples = trace.numsamples();

                let ydata = trace.to_vec_f64();

                let xdata: Vec<_> = (0..n_samples as i32).map(f64::from).collect();

                let mut data = Vec::with_capacity(n_samples as usize);
                for (x, y) in xdata.iter().zip(ydata.iter()) {
                    data.push((*x, *y));
                }

                let ymin = ydata
                    .iter()
                    .min_by(|a, b| a.partial_cmp(b).unwrap_or(Equal))
                    .unwrap();
                let ymax = ydata
                    .iter()
                    .max_by(|a, b| a.partial_cmp(b).unwrap_or(Equal))
                    .unwrap();

                let xmin = 0.0;
                let xmax = trace.numsamples() as f64;

                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(constraints.as_ref())
                    .split(size);

                let datasets = vec![Dataset::default()
                    .marker(symbols::Marker::Braille)
                    .style(Style::default().fg(Color::Yellow))
                    .graph_type(GraphType::Line)
                    .data(&data)];
                let chart = Chart::new(datasets)
                    .block(
                        Block::default()
                            .title(Span::styled(
                                format!(
                                    "{}.{}.{}.{}",
                                    tr.network(),
                                    tr.station(),
                                    tr.location(),
                                    tr.channel()
                                ),
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
                                    format!("{}", trace.start_time()),
                                    Style::default().add_modifier(Modifier::BOLD),
                                ),
                                Span::styled(
                                    format!("{}", trace.end_time()),
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
                } else if input == Key::Char('j')
                    && i_start + n_traces < trace_list.numtraces() as usize
                {
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
