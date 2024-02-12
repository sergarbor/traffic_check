use clap::Parser;
use cli::Cli;

pub mod cli;

fn main() {
    let args = Cli::parse();
    /*
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");
    */

    println!("Command: {} Value {}", args.command, args.value);

    if args.command == "check" {
        println!("THIS IS THE EXPECTED COMMAND")
    }
}
