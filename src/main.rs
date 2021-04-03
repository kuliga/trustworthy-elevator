mod lib;
use crate::lib::Elevator;

fn main() {
    println!("Hello, world!");
    let mut kapitol = Elevator::new();
    kapitol.panel();
    let x = 3;
}
