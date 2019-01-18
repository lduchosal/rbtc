use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;

fn main() {

    let n_workers = 128;
    let pool = ThreadPool::new(n_workers);

    let (tx, rx) : (Sender<u32>, Receiver<u32>) = channel();
    
    tx.send(0);
    while let Ok(data) = rx.recv() {

        println!("0. receiving: {}", data);
        let tx3 = tx.clone();
        let command2 = Walker {};
        pool.execute(move|| {
            command2.execute(tx3);
        });
    }
    
    pool.join();
}


pub struct Walker {}
impl Walker {

    pub fn execute(&self, send: Sender<u32>) {

        let mut i=0;
        while i < 10 {
            i = i+1;

            println!("Walker sending: {}",  i);
            send.send(i).expect("channel will be there waiting for the pool");
        }
    }
}

