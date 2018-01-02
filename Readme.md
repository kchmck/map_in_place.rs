# slice\_mip – Mutate a slice in place with a map function

[Documentation](https://docs.rs/slice_mip)

This crate provides a convenience utility for replacing the items in a slice with the
result of a map function applied to each item.

## Example

```rust
use slice_mip::MapInPlace;

let mut buf = [1, 2, 3, 4];
buf.map_in_place(|x| x * 2);

assert_eq!(buf, [2, 4, 6, 8]);
```

## Usage

This [crate](https://crates.io/crates/slice_mip) can be used through cargo by
adding it as a dependency in `Cargo.toml`:

```toml
[dependencies]
slice_mip = "1.0.0"
```
and importing it in the crate root:

```rust
extern crate slice_mip;
```

The provided slice method can then be used by importing the trait within individual
modules:

```rust
use slice_mip::MapInPlace;
```
