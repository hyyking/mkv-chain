/*!
A Markov chain is a stochastic model describing a sequence of possible events in which the
probability of each event depends only on the state attained in the previous event.

# Example

```rust
# extern crate markov_chain;
# use markov_chain::{MarkovChain3, linalg::{Matrix3, Vec3}};
# fn main() {
    let t_mat = Matrix3::new([  // Transition Matrix
        [0.9, 0.0, 0.1],
        [0.1, 0.3, 0.6],
        [0.0, 0.1, 0.9]],
    );
    let initial = Vec3::new([0.1, 0.3, 0.6]); // Inital State
    let mvc = MarkovChain3::from(t_mat, initial);
    assert_eq!(
        mvc.take_to(3),
        Vec3::new([0.12250000000000001, 0.11130000000000001, 0.7662])
    );
# }
```
*/

#![warn(missing_docs)]
#![allow(dead_code)]

#[macro_use]
mod macros;
pub use macros::*;

///! Algebra module for MarkovChains
pub mod linalg {
    vector!(Vec2: 2);
    vector!(Vec3: 3);
    vector!(Vec4: 4);
    vector!(Vec5: 5);
    vector!(Vec6: 6);

    matrix!(Matrix2[2, 2], Vec2);
    matrix!(Matrix3[3, 3], Vec3);
    matrix!(Matrix4[4, 4], Vec4);
    matrix!(Matrix5[5, 5], Vec5);
    matrix!(Matrix6[6, 6], Vec6);
}

markovchain!(MarkovChain2, linalg::Matrix2, linalg::Vec2);
markovchain!(MarkovChain3, linalg::Matrix3, linalg::Vec3);
markovchain!(MarkovChain4, linalg::Matrix4, linalg::Vec4);
markovchain!(MarkovChain5, linalg::Matrix5, linalg::Vec5);
markovchain!(MarkovChain6, linalg::Matrix6, linalg::Vec6);
