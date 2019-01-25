extern crate encoding;
extern crate rbtc;
extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::Editor;
use rbtc::cli::rbtc::RbtcPool;

use std::process::exit;
use std::sync::mpsc::channel;
use std::sync::mpsc;


enum Command {
    Help,
    Quit,
    SetAddr,
    Empty
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

        let mut rl = Editor::<()>::new();
        if rl.load_history("history.txt").is_err() {
            println!("No previous history.");
        }

        loop {
            let readline = rl.readline(">> ");
            match readline {
                Ok(line) => {
                    rl.add_history_entry(line.as_ref());
                    self.action(&line);
                },
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    break
                },
                Err(ReadlineError::Eof) => {
                    println!("CTRL-D");
                    break
                },
                Err(err) => {
                    println!("Error: {:?}", err);
                    break
                }
            }
            self.try_recv();
        }

        rl.save_history("history.txt").unwrap();
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
            "" => Ok(Command::Empty),
            _ => Err(line),
        };

        match command {
            Ok(Command::Help) => self.help(),
            Ok(Command::Quit) => self.quit(),
            Ok(Command::SetAddr) => self.set_addr(line),
            Ok(Command::Empty) => {},
            Err(s) => self.err(s),
        };
    }
    
    fn try_recv(&mut self) {

        while let Ok(recv) = self.rbtcpool.try_recv() {
            println!("recv: {:#?}", recv);
        }
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
        println!("  quit");
        println!("  help");
        println!("  setaddr 127.0.0.1:8333");
    }

    fn quit(&self) {
        println!("rbtc: have fun!");
        exit(0);
    }

    fn err(&self, invalid: &str) {
        println!("rbtc: {}: command not found", invalid);
    }

}