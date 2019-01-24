extern crate copperline;
extern crate encoding;
extern crate rbtc;

use std::process::exit;
use copperline::Encoding::Utf8;
use copperline::Copperline;
use rbtc::cli::rbtc::Rbtc;

enum Command {
    Help,
    Quit,
    SetAddr
}

struct RbtcCli {
    rbtc: Rbtc
}

fn main() {

    let cli = RbtcCli::new();
    cli.run();
}

impl RbtcCli {

    fn new() -> RbtcCli {
        let rbtc = Rbtc::new();
        RbtcCli {
            rbtc: rbtc
        }
    }

    fn run(&self) {
        let mut cl = Copperline::new();
        while let Ok(line) = cl.read_line("rbtc> ", Utf8) {
            self.action(&line);
            cl.add_history(line);
        };
    }

    fn action(&self, line: &str) {

        let tokens: Vec<&str> = line.split(" ").collect();
        let first = match tokens.get(0) {
            None => "help",
            Some(item) => item,
        };

        let command = match first {
            "quit" => Ok(Command::Quit),
            "exit" => Ok(Command::Quit),
            "setaddr" => Ok(Command::SetAddr),
            "?" => Ok(Command::Help),
            "help" => Ok(Command::Help),
            _ => Err(line),
        };

        match command {
            Ok(Command::Help) => self.help(),
            Ok(Command::Quit) => self.quit(),
            Ok(Command::SetAddr) => self.set_addr(line),
            Err(s) => self.err(s),
        };
    }

    fn set_addr(&self, line: &str) {

        println!("rbtc: {}", line);
        let addrs: Vec<&str> = line.split(" ").collect();
        addrs.remove(0);

        match addrs.get(0) {
            None => {
                self.help();
                return;
            },
            Some(addr) => {
                self.rbtc.set_addr(addr);
            }
        };

    }

    fn help(&self) {
        println!("rbtc 0.4.0 (q)");
        println!(" - quit : exit from rbtc");
        println!(" - help : this message");
        println!(" - addr hostname:port : set addr ");
    }

    fn quit(&self) {
        println!("rbtc: have fun!");
        exit(0);
    }

    fn err(&self, invalid: &str) {
        println!("rbtc: {}: command not found", invalid);
    }

}