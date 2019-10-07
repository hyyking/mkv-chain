/// Generate code for a vector with name and order
/// # Example:
/// ```rust
/// # #[cfg(feature = "serde")]
/// # #[macro_use]
/// # extern crate serde;
/// extern crate mkv_chain;
/// use mkv_chain::{vector};
///
/// vector!(MyVec3, 3);  // vector!(Name: Order)
/// vector!(MyVec5, 5);  // vector!(Name: Order)
///
/// fn main() {
///
///     let vec3_1 = MyVec3::new([0.0, 0.0, 0.0]);
///     let vec3_2 = MyVec3::new([1.0, 1.0, 1.0]);
///     assert_eq!(vec3_1 * vec3_2, 0.0);
///    
///     let vec5 = MyVec5::new([1.0, 2.0, 3.0, 4.0, 5.0]);
///     assert_eq!(
///         vec5.scale(2),
///         MyVec5::new([2.0, 4.0, 6.0, 8.0 ,10.0])
///     );
/// }
/// ```
#[macro_export]
macro_rules! vector {
    ($(#[$outer:meta])* $name:ident, $order:literal) => {
        $(#[$outer])*

        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        #[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
        pub struct $name([f64; $order]);

        impl $name {
            ///! Construct a vector from an array
            pub fn new(data: [f64; $order]) -> Self {
                Self(data)
            }
            ///! Generate an empty vector
            pub const fn zeros() -> Self {
                Self([0.0; $order])
            }
            ///! Downcast the vector to an owned array
            pub fn downcast(self) -> [f64; $order] {
                self.0
            }
            ///! Number of elements in the vector
            pub const fn len() -> usize {
                $order
            }
            ///! Turn the vector into a interator of it's values
            pub fn iter(&self) -> std::slice::Iter<f64> {
                self.0.iter()
            }
            ///! Scale the vector by the scaler
            pub fn scale<T: Into<f64> + Copy>(mut self, scaler: T) -> Self {
                self.0.iter_mut().for_each(|el| *el *= scaler.into());
                self
            }
        }

        impl ::core::ops::Index<usize> for $name {
            type Output = f64;
            fn index(&self, idx: usize) -> &Self::Output {
                &self.0[idx]
            }
        }
        impl ::core::ops::IndexMut<usize> for $name {
            fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
                &mut self.0[idx]
            }
        }

        impl ::core::ops::Mul for $name {
            type Output = f64;
            fn mul(self, other: $name) -> Self::Output {
                self.iter().zip(other.iter()).map(|(x, y)| x * y).sum()
            }
        }
        #[doc(hidden)]
        impl ::core::ops::Mul for &$name {
            type Output = f64;
            fn mul(self, other: &$name) -> Self::Output {
                self.iter().zip(other.iter()).map(|(x, y)| x * y).sum()
            }
        }
        #[doc(hidden)]
        impl ::core::ops::Mul for &mut $name {
            type Output = f64;
            fn mul(self, other: &mut $name) -> Self::Output {
                self.iter().zip(other.iter()).map(|(x, y)| x * y).sum()
            }
        }
    };
}

/// Generate code for a quare matrix with name, order and inner type
/// # Example:
/// ```rust
/// # #[cfg(feature = "serde")]
/// # #[macro_use]
/// # extern crate serde;
/// extern crate mkv_chain;
/// use mkv_chain::{vector, matrix};
///
/// vector!(MyVec3, 3); // vector!(name, order)
/// matrix!(MyMatrix3[3, 3], MyVec3); // matrix!(name[order], inner_type)
///
/// fn main() {
///    let mat3 = MyMatrix3::new(
///        [[1.0, 0.0, 0.0],
///         [0.0, 1.0, 0.0],
///         [0.0, 0.0, 1.0]]
///    );
///    let vec3 = MyVec3::new([1.0, 2.0, 3.0]);
///    assert_eq!(mat3 * vec3, vec3);
/// }
/// ```
#[macro_export]
macro_rules! matrix {
    ($(#[$outer:meta])* $name:ident[$row:literal,$col:literal], $inner:ident) => {
        $(#[$outer])*
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        #[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
        pub struct $name([$inner; $col]);
        impl $name {
            ///! Construct a Matrix from a row major representation
            pub fn new(data: [[f64; $row]; $col]) -> Self {
                let mut result = Self::zeros();
                data.iter().enumerate().for_each(|(i, row)| {
                    row.iter().enumerate().for_each(|(o, value)| {
                        result[(i, o)] = *value;
                    })
                });
                result
            }

            ///! Create an empty matrix
            pub const fn zeros() -> Self {
                Self([$inner::zeros(); $col])
            }

            ///! Shape of the matrix
            pub const fn shape() -> (usize, usize) {
                ($row, $col)
            }

            ///! Create a row major matrix from a column major one
            pub fn to_row_major(self) -> Self {
                let mut result = Self::zeros();
                let shape = Self::shape();
                (0..shape.0).for_each(|i| {
                    (0..shape.1).for_each(|o| {
                        result[(i, o)] = self[(o, i)];
                    });
                });
                result
            }

            ///! Turn the matrix into a interator of it's vectors
            pub fn iter(&self) -> std::slice::Iter<$inner> {
                self.0.iter()
            }
        }

        impl ::core::ops::Index<(usize, usize)> for $name {
            type Output = f64;
            fn index(&self, idx: (usize, usize)) -> &Self::Output {
                &self.0[idx.1][idx.0]
            }
        }
        impl ::core::ops::Index<usize> for $name {
            type Output = $inner;
            fn index(&self, idx: usize) -> &Self::Output {
                &self.0[idx]
            }
        }
        impl ::core::ops::IndexMut<(usize, usize)> for $name {
            fn index_mut(&mut self, idx: (usize, usize)) -> &mut Self::Output {
                &mut self.0[idx.1][idx.0]
            }
        }
        impl ::core::ops::IndexMut<usize> for $name {
            fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
                &mut self.0[idx]
            }
        }

        impl ::core::ops::Mul for $name {
            type Output = Self;
            fn mul(self, other: Self) -> Self::Output {
                let mut res = Self::zeros();
                other
                    .to_row_major()
                    .iter()
                    .enumerate()
                    .for_each(|(i, vec)| {
                        res[i] = &self * vec;
                    });
                res
            }
        }
        impl ::core::ops::Mul<$inner> for $name {
            type Output = $inner;
            fn mul(self, other: $inner) -> Self::Output {
                let mut result = $inner::zeros();
                self.iter().enumerate().for_each(|(i, vec)| {
                    result[i] = *vec * other;
                });
                result
            }
        }
        #[doc(hidden)]
        impl ::core::ops::Mul<&$inner> for &$name {
            type Output = $inner;
            fn mul(self, other: &$inner) -> Self::Output {
                let mut result = $inner::zeros();
                self.iter().enumerate().for_each(|(i, vec)| {
                    result[i] = other * vec;
                });
                result
            }
        }
        #[doc(hidden)]
        impl ::core::ops::Mul<$inner> for &mut $name {
            type Output = $inner;
            fn mul(self, other: $inner) -> Self::Output {
                let mut result = $inner::zeros();
                self.iter().enumerate().for_each(|(i, vec)| {
                    result[i] = *vec * other;
                });
                result
            }
        }
    };
}

/// Generate a markov chain from matrix and vec identifiers.
///
/// See generated ones for implemented methods and traits.
///
/// # Example:
/// ```
/// # #[cfg(feature = "serde")]
/// # #[macro_use]
/// # extern crate serde;
/// extern crate mkv_chain;
/// use mkv_chain::{markovchain, linalg::{Vec2, Matrix2}};
///
/// markovchain!(MyMarkovChain2, Matrix2, Vec2); // Same as markov_chain::MarkovChain2
///
/// fn main() {
///     let t_mat = Matrix2::new(
///         [[0.2, 0.8],
///          [0.6, 0.4]]
///     );
///     let initial = Vec2::new([0.0, 1.0]);
///
///     let mc = MyMarkovChain2::from(t_mat, initial);
///     println!("{:?}", mc.take_to(5));
/// }
/// ```
#[macro_export]
macro_rules! markovchain {
    ($(#[$outer:meta])* $name:ident, $trans:path, $init:path) => {
        $(#[$outer])*
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        #[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
        pub struct $name {
            init: $init,
            trans: $trans,
        }

        impl $name {
            /// Construct a Markov Chain from a transition matrix and an initial state
            pub fn from(trans: $trans, init: $init) -> Self {
                Self { trans, init }
            }

            /// Replace the transition matrix
            pub fn set_trans(&mut self, new_t: $trans) {
                self.trans = new_t;
            }

            /// Replace the initial state
            pub fn set_init(&mut self, new_t: $init) {
                self.init = new_t;
            }

            /// Run the chain until to a step.
            pub fn take_to(&self, state: usize) -> $init {
                let mut result = self.init;
                (0..state).for_each(|_| {
                    result = &self.trans * &result;
                });
                result
            }

            /// Check if the chain has an absorbing state. An absorbing state will loop on itself
            /// catching every bit coming to it.
            /// A state is considered absorbing if it has a probability of 1 to itself, there is a least one path leading to it.
            /// # Example:
            /// ```rust
            /// # extern crate mkv_chain;
            /// # use mkv_chain::{MarkovChain2, linalg::{Vec2, Matrix2}};
            /// # fn main() {
            ///     let t_mat = Matrix2::new(
            ///         [[1.0, 0.0],
            ///          [0.1, 0.9]]
            ///     );
            ///     let initial = Vec2::new([0.0, 1.0]);
            ///     // Initial state will converge to Vec2::new([1.0, 0.0])
            ///     assert!(
            ///         MarkovChain2::from(t_mat, initial).has_absorbing(),
            ///     );
            /// # }
            /// ```
            pub fn has_absorbing(&self) -> bool {
                for vec in self.trans.iter() {
                    if Self::could_absorb(vec) {
                        return true;
                    };
                }
                return false;
            }

            fn could_absorb(vec: &$init) -> bool {
                let mut has_one = false;
                let mut has_nz = false;
                vec.iter().for_each(|el| {
                    if *el == 1.0_f64 {
                        has_one = true;
                    } else if *el > 0.0f64 {
                        has_nz = true;
                    }
                });
                has_one && has_nz
            }
        }
    };
}
