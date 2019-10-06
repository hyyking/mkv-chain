///! A column major 3x3 square matrix
#[derive(Debug)]
pub struct Matrix3(pub(crate) [Vec3; 3]);

impl Matrix3 {
    ///! Construct a Matrix from a row major representation
    pub fn new(v0: [f64; 3], v1: [f64; 3], v2: [f64; 3]) -> Self {
        Self([
            Vec3([v0[0], v1[0], v2[0]]),
            Vec3([v0[1], v1[1], v2[1]]),
            Vec3([v0[2], v1[2], v2[2]]),
        ])
    }

    ///! Create a row major matrix from a column major one
    pub fn to_row_major(self) -> Self {
        // Self([Vec3([self[(0, 0)]]), Vec3([]), Vec3([])])
        Self([
            Vec3([self[(0, 0)], self[(1, 0)], self[(2, 0)]]),
            Vec3([self[(0, 1)], self[(1, 1)], self[(2, 1)]]),
            Vec3([self[(0, 2)], self[(1, 2)], self[(2, 2)]]),
        ])
    }

    ///! Turn the matrix into a interator of it's vectors
    pub fn iter(&self) -> std::slice::Iter<Vec3> {
        self.0.iter()
    }
}

impl std::ops::Index<(usize, usize)> for Matrix3 {
    type Output = f64;
    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        &self.0[idx.1][idx.0]
    }
}

///! Generate code for a vector with name and order
///! Example:
///! ```rust
///! vector!(Vec3, 3)  // vector!(Name, Order)
///! vector!(Vec5, 5)  // vector!(Name, Order)
///! main() {
///!    let vec3_1 = Vec3::new([0.0, 0.0, 0.0]);
///!    let vec3_2 = Vec3::new([1.0, 1.0, 1.0]);
///!    let mul = vec3_1 * vec3_2; // 0.0
///!    
///!    let vec5 = Vec5::new([1.0, 2.0, 3.0, 4.0, 5.0])
///! }
///! ```
#[macro_export]
macro_rules! vector {
    ($name:ident: $order:literal) => {
        ///! Vector with $order elements
        #[derive(Debug, PartialEq, Copy, Clone)]
        pub struct $name([f64; $order]);

        impl $name {
            ///! Construct a vector from an array
            pub fn new(data: [f64; $order]) -> Self {
                Self(data)
            }
            ///! Downcast the vector to an owned array
            pub fn downcast(self) -> [f64; $order] {
                self.0
            }

            ///! Turn the vector into a interator of it's values
            pub fn iter(&self) -> std::slice::Iter<f64> {
                self.0.iter()
            }
        }

        impl std::ops::Index<usize> for $name {
            type Output = f64;
            fn index(&self, idx: usize) -> &Self::Output {
                &self.0[idx]
            }
        }
        impl std::ops::IndexMut<usize> for $name {
            fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
                &mut self.0[idx]
            }
        }

        impl std::ops::Mul for $name {
            type Output = f64;
            fn mul(self, other: $name) -> Self::Output {
                self.iter().zip(other.iter()).map(|(x, y)| x * y).sum()
            }
        }
        impl std::ops::Mul for &$name {
            type Output = f64;
            fn mul(self, other: &$name) -> Self::Output {
                self.iter().zip(other.iter()).map(|(x, y)| x * y).sum()
            }
        }
        impl std::ops::Mul for &mut $name {
            type Output = f64;
            fn mul(self, other: &mut $name) -> Self::Output {
                self.iter().zip(other.iter()).map(|(x, y)| x * y).sum()
            }
        }
    };
}

vector!(Vec2: 2);
vector!(Vec3: 3);
vector!(Vec4: 4);
vector!(Vec5: 5);
vector!(Vec6: 6);
