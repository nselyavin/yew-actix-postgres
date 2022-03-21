# pharmacy-app
____
## Preparing
```
  cargo install trunk
  rustup target add wasm32-unknown-unknown
  cargo install --locked trunk
  cargo install wasm-bindgen-cli
```
____
## Deploy
backed
```
  cargo run --bin backend 
```
frontend
```
  trunk serve
```
