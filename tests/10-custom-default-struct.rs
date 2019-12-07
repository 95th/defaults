use defaults::Defaults;

struct NonDefault(usize);

#[derive(Defaults)]
struct Foo {
    x: usize,
    #[def = "NonDefault(10)"]
    non_default: NonDefault,
}

fn main() {}
