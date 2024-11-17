# Develop locally

install `bacon`:

```sh
cargo install --locked bacon
```

then run code expansion in watch mode:

```sh
bacon expand
```

and start hacking inside `tests/playground.rs`.

# Run tests:

```sh
cargo test
```

# Other commands:

To update the readme with the content from cargo doc:

```sh
cargo readme > README.md
```

# Code style

Rust files are formatted using

```sh
cargo fmt
```

Non-rust files are formatted using prettier

```sh
npx -y prettier --write .
```
