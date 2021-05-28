#![feature(iter_zip)]
extern crate clap;
use std::io;

// cli
use clap::load_yaml;
use clap::App;

mod tui;

use log::{info, LevelFilter};
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
// use trace::Input;

use miniseed::MSTraceList;

fn main() -> Result<(), io::Error> {
    TermLogger::init(
        LevelFilter::Trace,
        Config::default(),
        TerminalMode::Stdout,
        ColorChoice::Auto,
    )
    .unwrap();

    let cli_yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(cli_yaml).get_matches();

    let file = matches.value_of("FILENAME").unwrap();

    info!("Loading file {}", file);
    let mut traces = MSTraceList::new(file);
    traces.read();
    tui::start(traces).unwrap();

    Ok(())
}
