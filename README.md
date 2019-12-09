# `#[derive(Defaults)]`
A better `Default` derive macro.

[![Crate](https://img.shields.io/crates/v/defaults.svg)](https://crates.io/crates/defaults)


**Examples**:

You can provide default value for a non-default field.

```rust
#[derive(Defaults)]
pub struct Event {
    id: usize,
    name: String,
    #[def = "Instant::now()"]
    time: Instant,
}
```

You can also provide a different default value of an already default field.

```rust
#[derive(Defaults)]
pub struct Event {
    #[def = "4"]
    id: usize,
    name: String,
    #[def = "Instant::now()"]
    time: Instant,
}
```

You can even provide default value of enums:

```rust
#[derive(Defaults)]
#[def = "A"]
pub struct Classroom {
    A,
    B,
    C
}
```

But what about variants with data?

```rust
#[derive(Defaults)]
#[def = "A(100)"]
pub struct SomeEnum {
    A(usize),
    B { x: usize, y: usize },
    C
}
```
