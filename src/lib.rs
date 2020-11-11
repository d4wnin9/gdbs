extern crate clap;
extern crate dirs;

use std::error::Error;
use std::fs::File;
use std::process;
use std::io::prelude::*;
use std::process::Command;
use std::os::unix::process::CommandExt;


pub fn run(args: clap::ArgMatches) -> Result<(), Box<dyn Error>> {
    // .gdbs, .gbdibit へのパス
    let mut path_to_gdbs = dirs::home_dir().unwrap();
    path_to_gdbs.push(".gdbs");
    let mut path_to_gdbinit = dirs::home_dir().unwrap();
    path_to_gdbinit.push(".gdbinit");

    let gdb_type = gdb_type_check(&args);

    // .gdbs の読み込み
    let mut f = File::open(&path_to_gdbs)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    // source gdb
    let source_path = search(&gdb_type, &contents);

    // .gdbinit を書き換える
    let mut f = File::create(path_to_gdbinit)?;
    writeln!(f, "{}", source_path)?;

    // -i で解析したいfileを引数に取ったときはgdbを起動する
    if let Some(o) = args.value_of("filename") {
        run_gdb(o.to_string());
    };

    Ok(())
}

fn gdb_type_check(args: &clap::ArgMatches) -> String {
    // 条件にあったらそのgdbの種類を返す
    let mut gdb_type = String::new();
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

    // .gdbsからgdbの種類の一致するのを返す
    for line in contents.lines() {
        if line.contains(gdb_type) {
            source_path = line.to_string()
        }
    }

    source_path
}

fn run_gdb(filename: String) {
    // gdb -q ./filename
    let _ = Command::new("gdb")
        .arg("-q")
        .arg(filename)
        .exec();
}