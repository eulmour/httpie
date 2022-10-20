# httpie
One of the smallest http servers written in Rust

# Features
* Routing system inspired by express
* Single executable, cross-platform, zero dependencies
* Configurable via environment variables

# Build
## Linux, macOS, Windows
```console
cargo build --release
```
## Other linux, CentOS
```console
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
```

# Install
```console
cargo install --path .
```
You can install httpie as system service, follow the relevant instructions on the web

# Usage
* Run at default address 127.0.0.1:8080
```console
httpie
```
* Run at user-defined address
```console
httpie -a 0.0.0.0:12345
```
* More information
```console
httpie --help
```