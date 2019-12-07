use defaults::Defaults;

#[derive(Defaults)]
struct Foo {
    x: usize,
    y: usize,
}

fn main() {
    let foo = Foo::default();
    assert_eq!(0, foo.x);
    assert_eq!(0, foo.y);
}
