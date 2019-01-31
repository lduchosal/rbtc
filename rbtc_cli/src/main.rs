extern crate encoding;
extern crate rbtc;
extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::Editor;
use rbtc::cli::rbtc::Rbtc;

use futures::future::Future;


enum Command {
    Help,
    Quit,
    SetAddr,
    Empty,
    Connect
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

    fn run2(&mut self) {

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
        }
        rl.save_history("history.txt").unwrap();
    }

    fn run(&mut self) {

        loop {
            std::thread::sleep_ms(500);
            let line = "setaddr 127.0.0.1:8333";
            if let Ok(command) = self.action(&line) {
                match command {
                    Command::Quit => break,
                    _ => {}
                }
            }
        }
    }

    fn action(&mut self, line: &str) -> Result<Command, String> {

        let tokens: Vec<&str> = line.split(" ").collect();
        let first = match tokens.get(0) {
            None => "help",
            Some(item) => item,
        };

        let command = match first {
            "q" | "quit" | "exit" => Ok(Command::Quit),
            "." | "?" | "h" | "help"  => Ok(Command::Help),
            "a" | "setaddr" | "addr" => Ok(Command::SetAddr),
            "c" | "connect" | "conn" => Ok(Command::Connect),
            "" => Ok(Command::Empty),
            _ => Err(line.to_string()),
        };

        match &command {
            Ok(Command::Help) => self.help(),
            Ok(Command::Quit) => self.quit(),
            Ok(Command::SetAddr) => self.set_addr(line),
            Ok(Command::Connect) => self.connect(),
            Ok(Command::Empty) => {},
            Err(s) => self.err(s.clone()),
        };

        command
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
                match self.rbtc.set_addr(addr.to_string()).wait() {
                    Ok(result) => println!("setaddr [result: {:#?}]", result),
                    Err(err) => println!("setaddr [err: {:#?}]", err),
                }
            }
        };

    }

    fn connect(&mut self) {

        println!("connect");
        match self.rbtc.connect().wait() {
            Ok(result) => println!("connect [result: {:#?}]", result),
            Err(err) => println!("connect [err: {:#?}]", err),
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