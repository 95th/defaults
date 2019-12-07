use defaults::Defaults;

#[derive(Defaults)]
#[def = "A"]
enum Foo {
    A(usize),
    B,
}

fn main() {}
