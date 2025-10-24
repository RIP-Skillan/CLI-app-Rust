use clap::Parser;


fn main() {
    let args = CLI_app_Rust::Cli::parse();
    CLI_app_Rust::check_pattern(args);   
    
}
