extern crate copperline;
extern crate encoding;
extern crate rbtc;

use copperline::Encoding::Utf8;
use copperline::Copperline;
use rbtc::cli::rbtc::RbtcPool;

use std::process::exit;
use std::sync::mpsc::channel;
use std::sync::mpsc;


enum Command {
    Help,
    Quit,
    SetAddr
}

fn main() {

    let mut cli = RbtcCli::new();
    cli.run();
}

struct RbtcCli {
    rbtcpool: RbtcPool,
}

impl RbtcCli {

    fn new() -> RbtcCli {

        let rbtcpool = RbtcPool::new();
        RbtcCli {
            rbtcpool: rbtcpool,
        }
    }

    fn run(&mut self) {

        let mut cl = Copperline::new();
        while let Ok(line) = cl.read_line("rbtc> ", Utf8) {
            self.action(&line);
            cl.add_history(line);
        };
    }

    fn action(&mut self, line: &str) {

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

    fn set_addr(&mut self, line: &str) {

        let mut addrs: Vec<&str> = line.split(" ").collect();
        addrs.remove(0);

        match addrs.get(0) {
            None => {
                self.help();
                return;
            },
            Some(addr) => {
                println!(" setaddr");
                self.rbtcpool.set_addr(addr.to_string());
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