extern crate encoding;
extern crate rbtc;
extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::Editor;
use rbtc::cli::rbtc::Rbtc;

use std::sync::mpsc::{TryRecvError};


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
    rbtc: Rbtc,
}

impl RbtcCli {

    fn new() -> RbtcCli {

        let rbtc = Rbtc::new();
        RbtcCli {
            rbtc: rbtc,
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
                    if let Ok(command) = self.action(&line) {
                        match command {
                            Command::Quit => break,
                            _ => {}
                        }
                    }
                },
                Err(ReadlineError::Interrupted) 
                | Err(ReadlineError::Eof) => {
                    self.quit();
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

    fn action(&mut self, line: &str) -> Result<Command, String> {

        let tokens: Vec<&str> = line.split(" ").collect();
        let first = match tokens.get(0) {
            None => "help",
            Some(item) => item,
        };

        let command = match first {
            "q" | "quit" | "exit" => Ok(Command::Quit),
            "a" | "setaddr" | "addr" => Ok(Command::SetAddr),
            "." | "?" | "h" | "help"  => Ok(Command::Help),
            "" => Ok(Command::Empty),
            _ => Err(line.to_string()),
        };

        match &command {
            Ok(Command::Help) => self.help(),
            Ok(Command::Quit) => self.quit(),
            Ok(Command::SetAddr) => self.set_addr(line),
            Ok(Command::Empty) => {},
            Err(s) => self.err(s.clone()),
        };

        command
    }
    
    fn try_recv(&mut self) {
        let mut i = 0;
        loop {
             match self.rbtc.try_recv() {
                Ok(recv) => { 
                    println!("try_recv: [rcv: {:#?}]", recv); 
                    break; 
                },
                Err(TryRecvError::Empty) => { 
                    std::thread::sleep_ms(5);
                    if i > 2 {
                        break; 
                    }
                }
                Err(err) => println!("try_recv: [err: {:#?}]", err),
            }
            i += 1;
        }
    }

    fn set_addr(&mut self, line: &str) {

        let mut addrs: Vec<&str> = line.split(" ").collect();
        addrs.remove(0);

        match addrs.get(0) {
            None => {
                self.help();
            },
            Some(addr) => {
                println!("setaddr");
                match self.rbtc.set_addr(addr.to_string()) {
                    Ok(()) => println!("setaddr OK"),
                    Err(()) => println!("setaddr Err")
                }
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
        println!("Have fun!");
    }

    fn err(&self, invalid: String) {
        println!("{}: command not found", invalid);
    }

}