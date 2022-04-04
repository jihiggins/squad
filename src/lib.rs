#![feature(portable_simd)]
#![feature(array_zip)]

pub mod intersections;
pub mod ray;
pub mod shapes;
pub mod test_utils;
pub mod utils;
pub mod wide_intersections;

pub type Vec3 = mint::Vector3<f32>;
