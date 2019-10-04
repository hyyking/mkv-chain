extern crate markov_chain;

use markov_chain::{
    alg::{Matrix3, Vec3},
    MarkovChain3,
};

fn main() {
    let t_mat = Matrix3::new(
        // Transition Matrix
        /*  /     A   |    B   |    C  */
        /*A*/
        [0.9_f64, 0.0_f64, 0.1_f64],
        /*B*/ [0.1_f64, 0.3_f64, 0.6_f64],
        /*C*/ [0.0_f64, 0.1_f64, 0.9_f64],
    );
    let initial = Vec3::new([0.1, 0.3, 0.6]); // Inital State
    let mvc = MarkovChain3::from(t_mat, initial);
    assert_eq!(
        mvc.take_to(3),
        Vec3::new([0.12162, 0.09832199999999999, 0.6612132])
    );
}
