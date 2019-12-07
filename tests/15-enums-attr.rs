use defaults::Defaults;

#[derive(Defaults)]
#[def = "Foo::A"]
enum Foo {
    A,
    B,
}

fn main() {
    match Foo::default() {
        Foo::A => {}
        Foo::B => panic!("Expected A"),
    }
}
