# demo-dashboard documentation

## Dependencies

Install the following dependencies:

- [Rust](https://www.rust-lang.org/tools/install) to run the application on the server 
- Install the Dioxus tooling

```bash
cargo install dioxus-cli@0.5.0-alpha.0
```

- Install the `wasm32-unknown-unknown` target for Rust

```bash
rustup target add wasm32-unknown-unknown
```

- Create the demo-dashboard crate

*Note: you may have to search to find the directory you want to `cd` into.*
```bash
cargo new --bin demo-dashboard
cd demo-dashboard
```

- Run this command to add the web renderer dependencies to the `Cargo.toml` file

```bash
cargo add dioxus@0.5.0-alpha.0 --features web
```

## Running the application

- Run the application on the server

```bash
dx serve
```


## Reference

- Example Projects : https://github.com/DioxusLabs/example-projects/tree/master
- Rust : https://www.rust-lang.org/
- Dioxus : https://dioxuslabs.com/