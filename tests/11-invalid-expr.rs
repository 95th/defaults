use defaults::Defaults;

#[derive(Defaults)]
struct Foo {
    #[def = "1/"]
    x: usize,
}

fn main() {}
