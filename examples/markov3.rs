extern crate mkv_chain;

use mkv_chain::{
    linalg::{Matrix3, Vec3},
    MarkovChain3,
};

fn main() {
    // Transition Matrix
    let t_mat = Matrix3::new([[0.9, 0.0, 0.1], [0.1, 0.3, 0.6], [0.0, 0.1, 0.9]]);
    let initial = Vec3::new([0.1, 0.3, 0.6]); // Inital State
    let mvc = MarkovChain3::from(t_mat, initial);
    assert_eq!(
        mvc.take_to(3),
        Vec3::new([0.12250000000000001, 0.11130000000000001, 0.7662])
    );
}
