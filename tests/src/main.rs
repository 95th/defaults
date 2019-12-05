use defaults::Defaults;

fn foo() -> u8 {
    100
}

#[derive(Defaults)]
struct Y;

#[derive(Defaults)]
struct X(Y);

#[derive(Defaults)]
struct Foo {
    #[def = "foo() as usize"]
    f: usize,
    x: X,
}

fn main() {
    let f = Foo::default();
    println!("{}", f.f);
}
