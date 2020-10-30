extern crate clap;

use std::error::Error;
use std::fs::File;
use std::process;
use std::io::prelude::*;


pub fn run(args: clap::ArgMatches) -> Result<(), Box<dyn Error>> {
    let gdb_type = argparse(args);

    let mut f = File::open("~/.gdbs")?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let source_path = if gdb_type == "/bin/gdb" {
        gdb_type
    } else {
        search(&gdb_type, &contents)
    };

    let mut f = File::create("~/.gdbinit")?;
    writeln!(f, "{}", source_path)?;

    Ok(())
}

fn argparse(args: clap::ArgMatches) -> String {
    let mut gdb_type = "/bin/gdb".to_string();
    if let Some(arg) = args.value_of("type") {
        gdb_type = arg.to_string();
        gdb_type = match &*gdb_type {
            "p" | "peda" | "gdb-peda" => "peda".to_string(),
            "g" | "gef" => "gef".to_string(),
            "d" | "dbg" | "pwndbg" => "pwndbg".to_string(),
            _ => {
                eprintln!("Argument error: No such argument");
                process::exit(1);
            }
        };
    };

    gdb_type
}

fn search(gdb_type: &str, contents: &str) -> String {
    let mut source_path = String::new();

    for line in contents.lines() {
        if line.contains(gdb_type) {
            source_path = line.to_string()
        }
    }

    source_path
}