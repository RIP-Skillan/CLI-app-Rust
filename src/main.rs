use std::{fs::File, io::{BufRead, BufReader}};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn check_pattern (args: Cli) {

    let f = match std::fs::File::open(&args.path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };
    let reader = std::io::BufReader::new(f);

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

fn main() {
    let args = Cli::parse();
    check_pattern(args);

    //let content = std::fs::read_to_string(&args.path).expect("couldn't read file");
    
    
}
