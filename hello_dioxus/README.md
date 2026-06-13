# hello_dioxus

This is a simple Hello World application built with the [Dioxus](https://dioxuslabs.com/) framework.

## Prerequisites

```sh
brew install rustup
export PATH="$(dirname \"$(rustup which rustc)\"):$HOME/.cargo/bin:/opt/homebrew/bin:$PATH"
rustup default stable
rustup target add wasm32-unknown-unknown
cargo install dioxus-cli --version 0.6.3 --locked
```

## Usage

```sh
export PATH="$(dirname \"$(rustup which rustc)\"):$HOME/.cargo/bin:/opt/homebrew/bin:$PATH"
cd hello_dioxus
dx serve --platform web
```

## References

* [Dioxus](https://dioxuslabs.com/)
* [dioxus - crates.io](https://crates.io/crates/dioxus)
