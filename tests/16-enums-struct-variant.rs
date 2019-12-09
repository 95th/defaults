use defaults::Defaults;

#[derive(Defaults)]
#[def = "A { a: 10, b: 1000 }"]
enum Foo {
    A { a: u8, b: usize },
    B { x: u8, y: usize },
}

fn main() {
    if let Foo::A { a, b } = Foo::default() {
        assert_eq!(a, 10);
        assert_eq!(b, 1000);
    } else {
        panic!("Expected A");
    }
}
