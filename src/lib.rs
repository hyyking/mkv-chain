//! A Markov chain is a stochastic model describing a sequence of possible events in which the
//! probability of each event depends only on the state attained in the previous event.
//!
//! # Example
//!
//! ```rust
//! # extern crate mkv_chain;
//! # use mkv_chain::{MarkovChain3, linalg::{Matrix3, Vec3}};
//! # fn main() {
//!     let t_mat = Matrix3::new(  // Transition Matrix
//!       [[0.9, 0.0, 0.1],
//!        [0.1, 0.3, 0.6],
//!        [0.0, 0.1, 0.9]],
//!    );
//!    let initial = Vec3::new([0.1, 0.3, 0.6]); // Initial State
//!    let mvc = MarkovChain3::from(t_mat, initial);
//!    assert_eq!(
//!        mvc.take_to(3),
//!        Vec3::new([0.12250000000000001, 0.11130000000000001, 0.7662])
//!    );
//! # }
//! ```

#![warn(missing_docs)]

#[macro_use]
mod macros;
pub use macros::*;

/**
Algebra module for MarkovChains.

# Features:

- Vec[2-6]:     stack stored vectors
- Matrix[2-6]:  stack stored square matrixes

*/
pub mod linalg {
    vector!(
        ///! Stack stored vector with 2 elements
        Vec2,
        2
    );
    vector!(
        ///! Stack stored vector with 3 elements
        Vec3,
        3
    );
    vector!(
        ///! Stack stored vector with 4 elements
        Vec4,
        4
    );
    vector!(
        ///! Stack stored vector with 5 elements
        Vec5,
        5
    );
    vector!(
        ///! Stack stored vector with 6 elements
        Vec6,
        6
    );

    matrix!(
        /// 2 by 2 stack stored square matrix composed of two `Vec2`
        Matrix2[2, 2],
        Vec2
    );
    matrix!(
        /// 3 by 3 stack stored square matrix composed of three `Vec3`
        Matrix3[3, 3],
        Vec3
    );
    matrix!(
        /// 4 by 4 stack stored square matrix composed of four `Vec4`
        Matrix4[4, 4],
        Vec4
    );
    matrix!(
        /// 5 by 5 stack stored square matrix composed of five `Vec5`
        Matrix5[5, 5],
        Vec5
    );
    matrix!(
        /// 6 by 6 stack stored square matrix composed of six `Vec6`
        Matrix6[6, 6],
        Vec6
    );
}

markovchain!(
    ///! MarkovChain with two nodes.
    ///!
    ///! - Transition graph is a `Matrix2`
    ///! - Initial state is a `Vec2`
    MarkovChain2,
    linalg::Matrix2,
    linalg::Vec2
);

markovchain!(
    ///! MarkovChain with three nodes.
    ///!
    ///! - Transition graph is a `Matrix3`
    ///! - Initial state is a `Vec3`
    MarkovChain3,
    linalg::Matrix3,
    linalg::Vec3
);
markovchain!(
    ///! MarkovChain with four nodes.
    ///!
    ///! - Transition graph is a `Matrix4`
    ///! - Initial state is a `Vec4`
    MarkovChain4,
    linalg::Matrix4,
    linalg::Vec4
);
markovchain!(
    ///! MarkovChain with five nodes.
    ///!
    ///! - Transition graph is a `Matrix5`
    ///! - Initial state is a `Vec5`
    MarkovChain5,
    linalg::Matrix5,
    linalg::Vec5
);
markovchain!(
    ///! MarkovChain with six nodes.
    ///!
    ///! - Transition graph is a `Matrix6`
    ///! - Initial state is a `Vec6`
    MarkovChain6,
    linalg::Matrix6,
    linalg::Vec6
);
