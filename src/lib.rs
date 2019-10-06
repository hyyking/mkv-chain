/*!
A Markov chain is a stochastic model describing a sequence of possible events in which the
probability of each event depends only on the state attained in the previous event.

# Example

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
*/

#![warn(missing_docs)]
#![allow(dead_code)]

///! Algebra module for MarkovChains
pub mod alg;

///! MarkovChain with three nodes
pub struct MarkovChain3 {
    init: alg::Vec3,
    trans: alg::Matrix3,
}

impl MarkovChain3 {
    ///! Construct a Markov Chain.
    pub fn from(trans: alg::Matrix3, init: alg::Vec3) -> Self {
        Self { trans, init }
    }

    ///! Replace the transition matrix
    pub fn set_trans(&mut self, new_t: alg::Matrix3) {
        self.trans = new_t;
    }

    ///! Replace the initial state
    pub fn set_init(&mut self, new_t: alg::Vec3) {
        self.init = new_t;
    }

    ///! Run the chain until to a step.
    pub fn take_to(&self, state: usize) -> alg::Vec3 {
        let mut result = self.init;
        (0..state).for_each(|_| {
            self.trans.iter().enumerate().for_each(|(i, vec)| {
                result[i] = &result * vec;
            })
        });
        result
    }

    ///! Check if the chain has an absorbing state.
    pub fn has_absorbing_state(&self) -> bool {
        for vec in self.trans.iter() {
            if Self::could_absorb(vec) {
                return true;
            }
        }
        return false;
    }

    /// A state is considered absorbing if it has a probability of 1 to itself and there is a least
    /// one path leading to it.
    fn could_absorb(vec: &alg::Vec3) -> bool {
        let mut has_one = false;
        let mut has_nz = false;
        vec.iter().for_each(move |el| {
            if *el == 1.0_f64 {
                has_one = true;
            } else if *el != 0.0f64 {
                has_nz = true;
            }
        });
        has_one && has_nz
    }
}

#[cfg(test)]
mod vec3 {
    use super::*;
    #[test]
    fn test_vec_dot() {
        assert_eq!(
            alg::Vec3::new([1.0, 0.0, 1.0]) * alg::Vec3::new([2.0, 0.0, 1.0]),
            3.0_f64
        )
    }
}
