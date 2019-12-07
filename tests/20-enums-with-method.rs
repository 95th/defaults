use defaults::Defaults;

#[derive(Defaults)]
#[def = "foo_a(10)"]
enum Foo {
    A(usize),
    B,
}

fn foo_a(a: usize) -> Foo {
    Foo::A(a)
}

fn main() {
    if let Foo::A(a) = Foo::default() {
        assert_eq!(a, 10);
    } else {
        panic!("Expected A");
    }
}
