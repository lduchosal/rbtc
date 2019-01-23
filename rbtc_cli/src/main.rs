extern crate copperline;
extern crate encoding;
extern crate rbtc;

use std::process::exit;
use copperline::Encoding::Utf8;
use copperline::Copperline;
use rbtc::cli::rbtc::Rbtc;
use rbtc::cli::fsm::RbtcEvents;


enum Command {
    Help,
    Quit
}

struct RbtcCli {
    rbtc: Rbtc
}

fn main() {

    let cli = RbtcCli::new();
    let mut cl = Copperline::new();
    while let Ok(line) = cl.read_line("rbtc> ", Utf8) {
        cli.run(&line);
        cl.add_history(line);
    }
}

impl RbtcCli {

    fn new() -> RbtcCli {
        let rbtc = Rbtc::new();
        RbtcCli {
            rbtc: rbtc
        }
    }
    fn help(&self) {
        println!("rbtc 0.4.0 (q)");
        println!(" - quit : exit from rbtc");
        println!(" - help : this message");
    }

    fn quit(&self) {
        println!("rbtc: have fun!");
        exit(0);
    }

    fn err(&self, invalid: &str) {
        println!("rbtc: {}: command not found", invalid);
    }

    fn run(&self, line: &str) {

        let command = match line {
            "quit" => Ok(Command::Quit),
            "exit" => Ok(Command::Quit),
            "?" => Ok(Command::Help),
            "help" => Ok(Command::Help),
            _ => Err(line),
        };

        match command {
            Ok(Command::Help) => self.help(),
            Ok(Command::Quit) => self.quit(),
            Err(s) => self.err(s),
        };
    }
}