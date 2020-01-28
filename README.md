# Derive Dirty

Install [cargo-expand](https://github.com/dtolnay/cargo-expand) and run `cargo expand --test tests` to see the results.

## Examples

```rust

#[derive(Dirty)]
struct Model {
    id: Uuid,
    value: u16,
}

let dirty = DirtyModel { value: 42 };

```
### TODOS:

 - [x] parse macro input
 - [x] remove id field if present
 - [ ] remove custom fields?

