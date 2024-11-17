[![Crates.io](https://img.shields.io/crates/v/nest_struct.svg)](https://crates.io/crates/nest_struct)
[![CI](https://github.com/ZibanPirate/nest_struct/actions/workflows/ci.yml/badge.svg)](https://github.com/ZibanPirate/nest_struct/actions/workflows/ci.yml)
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

-   [x] deep nesting (no theoretical limit).
-   [x] nest `struct` inside another `struct`.
-   [x] nest `enum` inside another `enum`.
-   [x] nest `enum` inside a `struct` and vice-versa.
-   [x] inherit `derive` and other attribute macros from root `struct`.
-   [x] auto-generate inner `struct` names.
-   [ ] overwrite the auto-generated inner struct name.

Feature parity with native Rust code:

-   [x] `impl` block on inner `struct`s.
-   [ ] define `derive` and other attribute macros individually per inner `struct`.
-   [ ] define doc comments individually per inner `struct`.
-   [ ] useful complier error messages.
-   [x] support generic types.
-   [x] support lifetimes.

## Contributing

Contributions are welcome, please read [`CONTRIBUTING.md`](https://github.com/ZibanPirate/nest_struct/blob/main/CONTRIBUTING.md) to get started.

## License

Licensed under MIT (twitter: [@zibanpirate](https://twitter.com/zibanpirate)).
