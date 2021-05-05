use std::convert::From;

///
///
///
#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    arr: [f32; 4],
}

///
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
