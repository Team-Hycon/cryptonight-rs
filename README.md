# Rust-Cryptonight

[![Build Status](https://travis-ci.org/arigatodl/rust-cryptonight.svg?branch=master)](https://travis-ci.org/arigatodl/rust-cryptonight)

Rust-Cryptonight is a Rust wrapper around Cryptonight hash function from [Monero source code](https://github.com/monero-project/monero).

**This code is not stable yet. It is not recommended to use it in production.**

## Usage

To use Rust-Cryptonight, add the following to your Cargo.toml:

```toml
[dependencies]
rust-cryptonight = "^0.1"
```

and the following to your crate root:

```rust
extern crate cryptonight;
```

## Credits

Kudos to [Monero team](https://getmonero.org/community/team/) - C code from https://github.com/monero-project/monero 
