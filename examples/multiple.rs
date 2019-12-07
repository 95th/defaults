use defaults::Defaults;

#[derive(Defaults)]
struct X {
    #[def = "10"]
    x: usize,
}

fn main() {
    assert_eq!(10, X::default().x);
}
