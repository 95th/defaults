use defaults::Defaults;

#[derive(Defaults)]
struct Foo {
    #[def = "10"]
    #[def = "100"]
    x: usize,
}

fn main() {}
