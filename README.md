# nest_struct

Nest struct definitions with minimal syntax changes in Rust

Example

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
struct APIResponse_Abilities_Ability {
    name: String,
    url: String,
}
#[derive(serde::Deserialize)]
struct APIResponse_Abilities {
    ability: APIResponse_Abilities_Ability,
    is_hidden: bool,
    slot: u32,
}

#[derive(serde::Deserialize)]
struct APIResponse {
    id: u32,
    name: String,
    abilities: Vec<APIResponse_Abilities>,
}
```

for more examples, see the `tests/cases` directory.


License: MIT
