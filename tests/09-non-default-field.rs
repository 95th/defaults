use defaults::Defaults;

struct NonDefault;

#[derive(Defaults)]
struct Foo {
    x: usize,
    non_default: NonDefault,
}

fn main() {}
