# wlist_ui

The ui part of wlist.

## Build

1. Clone the repository and [`wlist_native`](https://github.com/wlist_org/wlist_native).
2. Prepare cargo tool: `cargo install flutter_rust_bridge_codegen`.
3. Run `flutter_rust_bridge_codegen integrate`.
4. Rollback all local changes.
5. Run `mkdir ./lib/generated/rust`.
6. Prepare cargo tool: `cargo install cargo-expand`.
7. Run `flutter_rust_bridge_codegen generate`.
8. Run `flutter create --platforms=windows,macos,linux,android,ios .`.
9. Finish and enjoy it!
