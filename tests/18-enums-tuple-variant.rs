use defaults::Defaults;

#[derive(Defaults)]
#[def = "Foo::A(10)"]
enum Foo {
    A(usize),
    B,
}

fn main() {
    if let Foo::A(a) = Foo::default() {
        assert_eq!(a, 10);
    } else {
        panic!("Expected A");
    }
}
