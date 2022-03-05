# LIBCLIC-SYS
*libclic-sys* is a Rust language wrapper sys-crate (or library) for the developed C library CLIC ( Command Line Interface written in C).
## Dependencies:
- cargo (```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh ```)
- bindgen (```cargo install bindgen```)

### Cargo + Rust-Lang installation (macOS + Linux):
```
Cargo:
curl https://sh.rustup.rs -sSf | sh

Cargo using rustup (recommended):
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
### Bindgen installation with Cargo (macOS + Linux):
```
cargo install bindgen
```

## Important Notes:
Bindings Rust file (*./bindings.rs*) was generated with the following command line:
```
bindgen src/include/clic/src/clic.h  -o src/bindings.rs
```