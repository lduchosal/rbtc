extern crate tokio;
extern crate futures;

use futures::*;
use std::fmt;

struct Hello {}

impl Future for Hello {
    type Item = String;
    type Error = ();

    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        Ok(Async::Ready("Hell".to_string()))
    }
}

struct Display1<T>(T);

impl<T> Future for Display1<T>
where T: Future,
      T::Item: fmt::Display
{
    type Item = ();
    type Error = T::Error;

    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        
        let result = match self.0.poll() {
            Err(err) => return Err(err),
            Ok(Async::NotReady) => return Ok(Async::NotReady),
            Ok(Async::Ready(value)) => value
        };

        println!("{}", result);
        Ok(Async::Ready(()))
    }
}

struct Run {}

impl Future for Run {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {

        let hello = Hello {};
        let world = World {}
        let display2 = Display2(display1);

    }
}

fn main() {

    let run = Run {};
    tokio::run(run);
}