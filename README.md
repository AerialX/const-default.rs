# ConstDefault Trait

A `Default`-like trait and derive macros for `const` evaluation contexts.

This crate defines the `ConstDefault` trait and implements it for
Rust primitives, prelude types, tuples and arrays. Furthermore it
provides a derive macro so that users can implement `ConstDefault`
easily for their custom types.

- 100% safe Rust
- `no_std` compatible
- Full macro hygiene
- No dependencies

## Usage

Add
```toml
[dependencies]
const-default = { version = "0.3", features = ["derive"] }
```
to your `Cargo.toml` and start using it.

## Example: Rust Primitive

```rust
use const_default::ConstDefault;

fn main() {
    assert_eq!(<i32 as ConstDefault>::DEFAULT, 0);
    assert_eq!(<Option<i32> as ConstDefault>::DEFAULT, None);
    assert_eq!(<String as ConstDefault>::DEFAULT, String::new());
    assert_eq!(<Vec<u8> as ConstDefault>::DEFAULT, Vec::new());
}
```

## Example: Derive

```rust
use const_default::ConstDefault;

#[derive(ConstDefault, Debug, Default, PartialEq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

fn main() {
    assert_eq!(
        <Color as ConstDefault>::DEFAULT,
        Color::default(),
    );
}
```
