use defaults::Defaults;

#[derive(Defaults)]
struct Foo {
    #[def]
    x: usize,
    y: usize,
}

fn main() {}
