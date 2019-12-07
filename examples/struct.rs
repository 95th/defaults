use defaults::Defaults;
use std::time::Instant;

#[derive(Debug, Defaults)]
pub struct Event {
    id: usize,
    name: String,
    #[def = "Instant::now()"]
    time: Instant,
}

fn main() {
    println!("{:?}", Event::default())
}
