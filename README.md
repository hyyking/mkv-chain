# rs-markov
The markov chain package for rust

## Docs

[here](https://hyyking.github.io/rs-markov)

## Example

```rust
# extern crate markov_chain;
# use markov_chain::{MarkovChain3, alg::{Matrix3, Vec3}};
# fn main() {
    let t_mat = Matrix3::new(  // Transition Matrix
        [0.9, 0.0, 0.1],
        [0.1, 0.3, 0.6],
        [0.0, 0.1, 0.9],
    );
    let initial = Vec3::new([0.1, 0.3, 0.6]); // Inital State
    let mvc = MarkovChain3::from(t_mat, initial);
    assert_eq!(
        mvc.take_to(3),
        Vec3::new([0.12162, 0.09832199999999999, 0.6612132])
    );
# }
```
