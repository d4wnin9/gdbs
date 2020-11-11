extern crate gdbs;
#[macro_use]
extern crate clap;

use std::process;
use clap::{App, Arg};


fn main() {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())

        .arg(Arg::with_name("type")
            .help("peda, gef, pwndbg")
            .required(true)
        )

        .arg(Arg::with_name("filename")
            .help("input file name")
            .short("i")
            .long("input")
            .takes_value(true)
        );

    let args = app.get_matches();

    if let Err(e) = gdbs::run(args) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}