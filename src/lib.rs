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
pub mod alg {
    ///! 3x3 Square Matrix
    #[derive(Debug)]
    pub struct Matrix3 {
        ///! A column major matrix is a collection of vectors
        pub(crate) data: [Vec3; 3],
    }

    impl Matrix3 {
        ///! Construct a Matrix from a row major representation
        pub fn new(v1: [f64; 3], v2: [f64; 3], v3: [f64; 3]) -> Self {
            Self {
                data: [
                    Vec3::new([v1[0], v2[0], v3[0]]),
                    Vec3::new([v1[1], v2[1], v3[1]]),
                    Vec3::new([v1[2], v2[2], v3[2]]),
                ],
            }
        }
    }

    ///! Vector with 3 elements
    #[derive(Debug, PartialEq, Copy, Clone)]
    pub struct Vec3 {
        ///! Collection of elements
        pub(crate) data: [f64; 3],
    }

    impl Vec3 {
        ///! Construct a vector from an array
        pub fn new(data: [f64; 3]) -> Self {
            Self { data }
        }
        ///! Downcast the vector to an owned array
        pub fn downcast(self) -> [f64; 3] {
            self.data
        }
    }

    impl std::ops::Mul for Vec3 {
        type Output = f64;
        fn mul(self, other: Vec3) -> Self::Output {
            self.data.iter().zip(&other.data).map(|(x, y)| x * y).sum()
        }
    }
    impl std::ops::Mul for &Vec3 {
        type Output = f64;
        fn mul(self, other: &Vec3) -> Self::Output {
            self.data.iter().zip(&other.data).map(|(x, y)| x * y).sum()
        }
    }
    impl std::ops::Mul for &mut Vec3 {
        type Output = f64;
        fn mul(self, other: &mut Vec3) -> Self::Output {
            self.data.iter().zip(&other.data).map(|(x, y)| x * y).sum()
        }
    }
}

///! MarkovChain with three nodes
pub struct MarkovChain3 {
    init: alg::Vec3,
    trans: alg::Matrix3,
}

impl MarkovChain3 {
    ///! Construct a Markov Chain
    pub fn from(trans: alg::Matrix3, init: alg::Vec3) -> Self {
        Self { trans, init }
    }
    ///! Check if the chain has an absorbing state
    pub fn has_absorbing_state(&self) -> bool {
        for vec in &self.trans.data {
            if (vec.data[0] == 1.0_f64) | (vec.data[1] == 1.0_f64) | (vec.data[2] == 1.0_f64) {
                return true;
            }
        }
        return false;
    }
    ///! Run the chain until to a step
    pub fn take_to(&self, state: usize) -> alg::Vec3 {
        let mut result = self.init;
        (0..state).for_each(|_| {
            self.trans.data.iter().enumerate().for_each(|(i, vec)| {
                result.data[i] = &result * vec;
            })
        });
        result
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
