[![Crates.io](https://img.shields.io/crates/v/nest_struct.svg)](https://crates.io/crates/nest_struct)
[![Workflow Status](https://github.com/ZibanPirate/nest_struct/workflows/main/badge.svg)](https://github.com/ZibanPirate/nest_struct/actions?query=workflow%3A%22main%22)
![Maintenance](https://img.shields.io/badge/maintenance-activly--developed-brightgreen.svg)

# nest_struct

Nest struct and enum definitions with minimal syntax changes in Rust

### Example

```rust
use nest_struct::nest_struct;

// Define a struct with nested struct definitions all in one place
// with minimal syntax changes.
#[nest_struct]
#[derive(serde::Deserialize)]
struct APIResponse {
    id: u32,
    name: String,
    abilities: Vec<nest! {
        ability: nest! {
            name: String,
            url: String,
        },
        is_hidden: bool,
        slot: u32,
    }>
};

let body = reqwest::blocking::get("https://pokeapi.co/api/v2/pokemon/ditto").unwrap().text().unwrap();
let api_response: APIResponse = serde_json::from_str(&body).unwrap();

assert_eq!(api_response.name, "ditto");
// Access nested struct fields
assert_eq!(api_response.abilities.first().unwrap().ability.name, "limber");
```

The expanded code for the struct above would look like this:

```rust
#[derive(serde::Deserialize)]
struct APIResponseAbilitiesAbility {
    name: String,
    url: String,
}

#[derive(serde::Deserialize)]
struct APIResponseAbilities {
    ability: APIResponseAbilitiesAbility,
    is_hidden: bool,
    slot: u32,
}

#[derive(serde::Deserialize)]
struct APIResponse {
    id: u32,
    name: String,
    abilities: Vec<APIResponseAbilities>,
}
```

For more examples, see the [`./tests/cases`](https://github.com/ZibanPirate/nest_struct/tree/main/tests/cases) directory.

### Features

- [x] deep nesting (no theoretical limit).
- [x] nest `struct` inside another `struct`.
- [x] nest `enum` inside another `enum`.
- [x] nest `enum` inside a `struct` and vice-versa.

Feature parity with native Rust code:

- [x] `impl` block on inner `struct`s.
- [x] inherit `derive` and other attribute macros from root `struct`.
- [ ] define `derive` and other attribute macros individually per inner `struct`.
- [x] support generic types.
- [x] support lifetimes.


License: MIT
