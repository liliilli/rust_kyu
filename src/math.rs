use std::{convert::From, fmt::Debug, ops};

/// Represent vector type which contains 3 elements.
///
/// Actually, this type is implemented to have 4 elements
/// for utilizing SIMD and optimization, but last element is hidden.
///
/// To get a actual vector which have only 3 elements inside, convert it into struct `FitVec3`.
///
/// # Examples
///
/// ```
/// use kyu::math::Vec3;
///
/// let mut vec = Vec3::default();
/// assert_eq!(vec, Vec3::new(0f32, 0f32, 0f32));
/// ```
#[derive(Copy, Clone)]
pub struct Vec3 {
    arr: [f32; 4],
}

impl Vec3 {
    /// Create new `Vec3` value.
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            arr: [x, y, z, 0f32],
        }
    }

    /// Create new `Vec3` value from array that has 3 elements.
    pub fn from_array(arr: [f32; 3]) -> Self {
        Self {
            arr: [arr[0], arr[1], arr[2], 0f32],
        }
    }

    /// Create x unit `(1, 0, 0)` vector.
    pub fn unit_x() -> Self {
        Self::new(0f32, 1f32, 0f32)
    }

    /// Create y unit `(0, 1, 0)` vector.
    pub fn unit_y() -> Self {
        Self::new(0f32, 1f32, 0f32)
    }

    /// Create z unit `(0, 0, 1)` vector.
    pub fn unit_z() -> Self {
        Self::new(0f32, 1f32, 0f32)
    }
}

impl Default for Vec3 {
    /// Create zero vector.
    fn default() -> Self {
        Self::new(0f32, 0f32, 0f32)
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.arr[0] == other.arr[0] && self.arr[1] == other.arr[1] && self.arr[2] == other.arr[2]
    }
}

impl From<FitVec3> for Vec3 {
    fn from(vec: FitVec3) -> Self {
        Self::from_array(vec.arr)
    }
}

impl Debug for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Vec3 {{x: {:.3}, y: {:.3}, z: {:.3}}}",
            self.arr[0], self.arr[1], self.arr[2]
        )
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.arr[index]
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            arr: [
                self.arr[0] + other.arr[0],
                self.arr[1] + other.arr[1],
                self.arr[2] + other.arr[2],
                self.arr[3] + other.arr[3],
            ],
        }
    }
}

impl ops::Add<f32> for Vec3 {
    type Output = Self;

    fn add(self, other: f32) -> Self {
        Self {
            arr: [
                self.arr[0] + other,
                self.arr[1] + other,
                self.arr[2] + other,
                self.arr[3] + 0f32,
            ],
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.arr[0] += rhs.arr[0];
        self.arr[1] += rhs.arr[1];
        self.arr[2] += rhs.arr[2];
        self.arr[3] += rhs.arr[3];
    }
}

impl ops::AddAssign<f32> for Vec3 {
    fn add_assign(&mut self, rhs: f32) {
        self.arr[0] += rhs;
        self.arr[1] += rhs;
        self.arr[2] += rhs;
        self.arr[3] += 0f32;
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            arr: [
                self.arr[0] - other.arr[0],
                self.arr[1] - other.arr[1],
                self.arr[2] - other.arr[2],
                self.arr[3] - other.arr[3],
            ],
        }
    }
}

impl ops::Sub<f32> for Vec3 {
    type Output = Self;

    fn sub(self, other: f32) -> Self {
        Self {
            arr: [
                self.arr[0] - other,
                self.arr[1] - other,
                self.arr[2] - other,
                self.arr[3] - 0f32,
            ],
        }
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.arr[0] -= rhs.arr[0];
        self.arr[1] -= rhs.arr[1];
        self.arr[2] -= rhs.arr[2];
        self.arr[3] -= rhs.arr[3];
    }
}

impl ops::SubAssign<f32> for Vec3 {
    fn sub_assign(&mut self, rhs: f32) {
        self.arr[0] -= rhs;
        self.arr[1] -= rhs;
        self.arr[2] -= rhs;
        self.arr[3] -= 0f32;
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            arr: [
                self.arr[0] * other.arr[0],
                self.arr[1] * other.arr[1],
                self.arr[2] * other.arr[2],
                self.arr[3] * other.arr[3],
            ],
        }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self {
            arr: [
                self.arr[0] * other,
                self.arr[1] * other,
                self.arr[2] * other,
                self.arr[3] * other,
            ],
        }
    }
}

impl ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.arr[0] *= rhs.arr[0];
        self.arr[1] *= rhs.arr[1];
        self.arr[2] *= rhs.arr[2];
        self.arr[3] *= rhs.arr[3];
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.arr[0] *= rhs;
        self.arr[1] *= rhs;
        self.arr[2] *= rhs;
        self.arr[3] *= rhs;
    }
}

/// Represent vector type but actually have only 3 elments unlike `Vec3`.
///
///
#[derive(Copy, Clone, Debug)]
pub struct FitVec3 {
    arr: [f32; 3],
}

impl From<Vec3> for FitVec3 {
    fn from(vec: Vec3) -> Self {
        Self {
            arr: [vec.arr[0], vec.arr[1], vec.arr[2]],
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn vector3_test() {
        let vec3 = Vec3::new(13f32, 29f32, 53f32);
        let fitvec3: FitVec3 = vec3.into();
        let reconv_vec3: Vec3 = fitvec3.into();

        println!("{:?}", vec3);
        println!("{:?}", fitvec3);
        println!("{:?}", reconv_vec3);
    }
}
