# my-sandbox-rust

## rustup

To update the rust tools.

```bash
rustup check
rustup update
```

## cargo

```bash
cargo new my-new-project
```

and in the root folder, where the Cargo.toml is:

```bash
cargo build
cargo run
cargo check
cargo clippy
```

Otherwise, manually:

```bash
rustc --out-dir ./out/hello-world.exe ./src/main.rs
```
