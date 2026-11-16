echo building

cargo build --profile wasm-release --target wasm32-unknown-unknown

echo bindings

wasm-bindgen --out-dir ./out --target web ./target/wasm32-unknown-unknown/wasm-release/cozy-landscape.wasm
