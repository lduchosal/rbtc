extern crate copperline;
extern crate encoding;

use std::process::exit;
use copperline::Encoding::Utf8;
use copperline::Copperline;


enum Command {
    Help,
    Quit
}
fn main() {

    let mut cl = Copperline::new();
    while let Ok(line) = cl.read_line("rbtc> ", Utf8) {
        
        let command = parse(&line);
        match command {
            Ok(Command::Help) => help(),
            Ok(Command::Quit) => quit(),
            Err(s) => err(s),
        };

        cl.add_history(line);
    }
}

fn help() {
    println!("rbtc 0.4.0 (q)");
    println!(" - quit : exit from rbtc");
    println!(" - help : this message");
}

fn quit() {
    println!("rbtc: have fun!");
    exit(0);
}

fn err(invalid: &str) {
    println!("rbtc: {}: command not found", invalid);
}

fn parse(line: &str) -> Result<Command, &str> {

    match line {
        "quit" => Ok(Command::Quit),
        "exit" => Ok(Command::Quit),
        "?" => Ok(Command::Help),
        "help" => Ok(Command::Help),
        _ => Err(line),
    }
}
