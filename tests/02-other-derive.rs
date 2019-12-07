use defaults::Defaults;

#[derive(Defaults, Debug)]
struct Foo {
    x: usize,
    y: usize,
}

fn main() {
    let foo = Foo::default();
    let _ = format!("{:?}", foo);
}
