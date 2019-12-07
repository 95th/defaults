use defaults::Defaults;

#[derive(Defaults)]
#[def = "Foo::A"]
enum Foo {
    A(usize),
    B,
}

fn main() {}
