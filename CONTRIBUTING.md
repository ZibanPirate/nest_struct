# Develop locally

install `bacon`:

```sh
cargo install --locked bacon             
```

then run code expansion in watch mode:

```sh
bacon expand
```

and start hacking inside `tests/develop.rs`.

# Run tests:

```sh
cargo test
```

# Other commands:

To update the readme with the content from cargo doc:

```sh
cargo readme > README.md
```
