use std::thread;

use crossbeam::channel::unbounded;

#[macro_use]
extern crate crossbeam;

fn main() {
  let (tx, rx) = unbounded();
  thread::spawn(move || {
    tx.send(42).unwrap();
  });

  select! {recv(rx) -> msg => println!("{:?}", msg)}
}
