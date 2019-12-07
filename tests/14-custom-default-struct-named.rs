use defaults::Defaults;

struct NonDefault {
    a: usize,
}

#[derive(Defaults)]
struct Foo {
    x: usize,
    #[def = "NonDefault { a: 10 }"]
    non_default: NonDefault,
}

fn main() {}
