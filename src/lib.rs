use clap::Parser;
use std::{fs::File, io::{BufRead, BufReader}};

#[derive(Parser)]
pub struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

pub fn check_pattern (args: Cli) {

    let f = match File::open(&args.path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };
    let reader = BufReader::new(f);

    for res in reader.lines() {
        let line = match res{
            Ok(line) => line,
            Err(e) => e.to_string()
        };

        if line.contains(&args.pattern){
            println!("{}", line);
        }
    };
}