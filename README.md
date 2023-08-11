# Ocex Coupon Signature
JS/Wasm library for coupon activation signatures.

### How to build
You need `wasm-pack` to start building

``` bash
cargo install wasm-pack
```

### Build NodeJS package
```
wasm-pack build --target nodejs --out-dir pkg_node --release
```

### Build Web JS package

```
wasm-pack build --target bundler --out-dir pkg_web --release
```
