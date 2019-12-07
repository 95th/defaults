use defaults::Defaults;

#[derive(Debug, Defaults)]
#[def = "Self::B"]
pub enum Classroom {
    A,
    B,
    C,
}

fn main() {
    println!("{:?}", Classroom::default());
}
