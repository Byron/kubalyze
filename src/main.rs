#[macro_use]
extern crate clap;
extern crate failure;
extern crate kubealyze;

use clap::{App, Arg};
use std::path::Path;
use failure::ResultExt;
use std::fs::File;
use std::io::{stderr, Write};

fn main() {
    let mut app: App = app_from_crate!();
    app = app.arg(
        Arg::with_name("file")
            .required(true)
            .multiple(true)
            .takes_value(true)
            .help("The json file to process"),
    );

    let matches = app.get_matches();
    let mut state = kubealyze::State::default();

    for path in matches
        .values_of_os("file")
        .expect("clap to work")
        .map(|os| Path::new(os))
    {
        if let Err(e) = File::open(path)
            .context(format!("Could not open file at '{}'", path.display()))
            .map_err(Into::into)
            .and_then(|mut f| state.inject(&mut f))
        {
            writeln!(stderr(), "{}", e).ok();
        }
    }
}
