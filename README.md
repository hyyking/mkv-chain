# *mkv_chain* [![Build Status](https://travis-ci.org/hyyking/markov-chain-rs.svg?branch=master)](https://travis-ci.org/hyyking/markov-chain-rs) [![Latest Version](https://img.shields.io/crates/v/mkv_chain.svg)](https://crates.io/crates/mkv_chain) [![Rust Documentation](https://docs.rs/mkv_chain/badge.svg)](https://docs.rs/mkv_chain)

Stochastic Oriented Markov Chain Crate

```toml
[dependencies]
mkv_chain="0.3.0"
```

## Example

```rust
extern crate mkv_chain;
use mkv_chain::{MarkovChain3, linalg::{Matrix3, Vec3}};
fn main() {
  let t_mat = Matrix3::new(  // Transition Matrix
    [[0.9, 0.0, 0.1],
     [0.1, 0.3, 0.6],
     [0.0, 0.1, 0.9]],
 );
 let initial = Vec3::new([0.1, 0.3, 0.6]); // Initial State
 let mvc = MarkovChain3::from(t_mat, initial);
 assert_eq!(
     mvc.take_to(3),
     Vec3::new([0.12250000000000001, 0.11130000000000001, 0.7662])
 );
}
```
