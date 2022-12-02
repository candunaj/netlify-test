# Build command

```
cargo build --target wasm32-wasi --release
wasm-pack build --target web
```

then copy rust_rewrite.wasm to edge-functions directory

## Run tests with debug output
```
cargo test -- --nocapture
```
## Run netlify locally
netlify dev