extern crate copperline;
extern crate encoding;

use copperline::Encoding::Utf8;
use copperline::Copperline;

fn main() {

    let mut cl = Copperline::new();
    while let Ok(line) = cl.read_line("rbtc> ", Utf8) {
        println!("Line: {}", line);
        cl.add_history(line);
    }
}
