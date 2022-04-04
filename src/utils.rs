use std::simd::{LaneCount, Simd, SupportedLaneCount};

pub fn simd_permutations<
    const LANES: usize,
    const WIDTH: usize,
    const WIDTH2: usize,
    T,
>(
    input_a: &[Vec<f32>; WIDTH],
    input_b: &[Vec<f32>; WIDTH2],
    mut f: T,
) where
    LaneCount<LANES>: SupportedLaneCount,
    T: FnMut([Simd<f32, LANES>; WIDTH], [Simd<f32, LANES>; WIDTH]),
    [Simd<f32, LANES>; WIDTH]: Default,
{
    let zero = [0.0; LANES];
    let count = input_a[0].len();
    let mut simd_a: [Simd<f32, LANES>; WIDTH] = Default::default();
    let mut simd_b: [Simd<f32, LANES>; WIDTH] = Default::default();

    let same_inputs = input_a.as_ptr() == input_b.as_ptr();

    let i_len = (count as f32 / LANES as f32).ceil() as usize;
    for i in 0..i_len {
        for k in 0..WIDTH {
            let overflow = ((i + 1) * LANES) as i64 - count as i64;
            let overflow = overflow.max(0) as usize;
            let slice = &input_a[k][i * LANES..((i + 1) * LANES).min(count)];
            simd_a[k] = Simd::from_slice(&[slice, &zero[..overflow]].concat());
        }

        let j_offset = if same_inputs {
            (i + 1) * LANES - (LANES - 1)
        } else {
            0
        };

        for j in j_offset..count {
            for k in 0..WIDTH {
                simd_b[k] = simd_b[k].rotate_lanes_right::<1>();
                simd_b[k][0] = input_b[k][j];
            }

            f(simd_a, simd_b);
        }
    }
}
