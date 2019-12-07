use defaults::Defaults;

#[derive(Defaults)]
#[def = "Foo::A { a: 10 }"]
enum Foo {
    A { a: u8, b: usize },
    B { x: u8, y: usize },
}

fn main() {}
