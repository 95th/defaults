use defaults::Defaults;

#[derive(Defaults)]
struct Foo {
    #[def = 10]
    x: usize,
    y: usize,
}

fn main() {}
