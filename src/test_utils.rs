use crate::shapes::Aabb;
use std::simd::{Simd, SupportedLaneCount};

pub fn generate_test_data<const WIDTH: usize>(
    count: usize,
    mut generator: impl FnMut(usize) -> [f32; WIDTH],
) -> ([Vec<f32>; 6], Vec<[[f32; 3]; 2]>) {
    let mut simd_aabbs: [Vec<f32>; 6] = [
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    ];
    for i in 0..count {
        let aabb = generator(i);
        simd_aabbs[0].push(aabb[0]);
        simd_aabbs[1].push(aabb[1]);
        simd_aabbs[2].push(aabb[2]);
        simd_aabbs[3].push(aabb[3]);
        simd_aabbs[4].push(aabb[4]);
        simd_aabbs[5].push(aabb[5]);
    }

    let mut aabbs = Vec::with_capacity(count);
    for i in 0..count {
        let a = [
            [simd_aabbs[0][i], simd_aabbs[1][i], simd_aabbs[2][i]],
            [simd_aabbs[3][i], simd_aabbs[4][i], simd_aabbs[5][i]],
        ];
        aabbs.push(a);
    }
    (simd_aabbs, aabbs)
}
