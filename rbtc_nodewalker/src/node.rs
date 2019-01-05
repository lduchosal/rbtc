extern crate time;
use time::Timespec;

#[derive(Debug)]
pub struct Node {
    pub id: i32,
    pub ip: String,
    pub src: String,
    pub creation: Timespec
}
