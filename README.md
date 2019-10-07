# *mkv_chain*

Stochastic Oriented Markov Chain Crate

## [Docs](https://hyyking.github.io/markov-chain-rs)

## Install

In cargo.toml
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
