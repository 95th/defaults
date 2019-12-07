use defaults::Defaults;

fn calc_x() -> usize {
    10
}

#[derive(Defaults)]
struct Foo {
    #[def = "calc_x()"]
    x: usize,
    y: usize,
}

fn main() {
    let foo = Foo::default();
    assert_eq!(10, foo.x);
    assert_eq!(0, foo.y);
}
