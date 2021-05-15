extern crate clap;

use std::io;

// cli
use clap::load_yaml;
use clap::App;

mod trace;
mod tui;

fn main() -> Result<(), io::Error> {
    let cli_yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(cli_yaml).get_matches();

    let file = matches
        .value_of("trace")
        .unwrap_or("../libmseed-sys/tests/sample.miniseed");
    trace::read_mseed(&file);

    tui::start();

    Ok(())
}
