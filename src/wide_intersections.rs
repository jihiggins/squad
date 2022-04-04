use float_ord::FloatOrd;
use std::simd::{LaneCount, Mask, Simd, SupportedLaneCount};

pub fn aabb_aabb<const LANES: usize>(
    [a_min_x, a_min_y, a_min_z, a_max_x, a_max_y, a_max_z]: [Simd<f32, LANES>; 6],
    [b_min_x, b_min_y, b_min_z, b_max_x, b_max_y, b_max_z]: [Simd<f32, LANES>; 6],
) -> Mask<i32, LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    let min_x_test = a_min_x.lanes_le(b_max_x);
    let min_y_test = a_min_y.lanes_le(b_max_y);
    let min_z_test = a_min_z.lanes_le(b_max_z);
    let max_x_test = a_max_x.lanes_ge(b_min_x);
    let max_y_test = a_max_y.lanes_ge(b_min_y);
    let max_z_test = a_max_z.lanes_ge(b_min_z);
    min_x_test & min_y_test & min_z_test & max_x_test & max_y_test & max_z_test
}

pub fn aabb_point<const LANES: usize>(
    [a_min_x, a_min_y, a_min_z, a_max_x, a_max_y, a_max_z]: [Simd<f32, LANES>; 6],
    [p_x, p_y, p_z]: [Simd<f32, LANES>; 3],
) -> Mask<i32, LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    let min_x_test = p_x.lanes_ge(a_min_x);
    let min_y_test = p_y.lanes_ge(a_min_y);
    let min_z_test = p_z.lanes_ge(a_min_z);
    let max_x_test = p_x.lanes_le(a_max_x);
    let max_y_test = p_y.lanes_le(a_max_y);
    let max_z_test = p_z.lanes_le(a_max_z);
    min_x_test & min_y_test & min_z_test & max_x_test & max_y_test & max_z_test
}

pub fn aabb_sphere<const LANES: usize>(
    [a_min_x, a_min_y, a_min_z, a_max_x, a_max_y, a_max_z]: [Simd<f32, LANES>; 6],
    [s_x, s_y, s_z, s_radius]: [Simd<f32, LANES>; 4],
) -> Mask<i32, LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    let s_x = (s_x - a_min_x).abs();
    let s_y = (s_y - a_min_y).abs();
    let s_z = (s_z - a_min_z).abs();
    let c_x = a_max_x - a_min_x + s_radius;
    let c_y = a_max_y - a_min_y + s_radius;
    let c_z = a_max_z - a_min_z + s_radius;
    let x_test = s_x.lanes_le(c_x);
    let y_test = s_y.lanes_le(c_y);
    let z_test = s_z.lanes_le(c_z);
    x_test & y_test & z_test
}

pub fn sphere_sphere<const LANES: usize>(
    [a_x, a_y, a_z, a_radius]: [Simd<f32, LANES>; 4],
    [b_x, b_y, b_z, b_radius]: [Simd<f32, LANES>; 4],
) -> Mask<i32, LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    let dx = a_x - b_x;
    let dy = a_y - b_y;
    let dz = a_z - b_z;
    let distance_squared = dx * dx + dy * dy + dz * dz;
    let radius_sum = a_radius + b_radius;
    let radius_sum_squared = radius_sum * radius_sum;
    distance_squared.lanes_le(radius_sum_squared)
}

pub fn sphere_point<const LANES: usize>(
    [s_x, s_y, s_z, s_radius]: [Simd<f32, LANES>; 4],
    [p_x, p_y, p_z]: [Simd<f32, LANES>; 3],
) -> Mask<i32, LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    let dx = s_x - p_x;
    let dy = s_y - p_y;
    let dz = s_z - p_z;
    let distance_squared = dx * dx + dy * dy + dz * dz;
    let radius_squared = s_radius * s_radius;
    distance_squared.lanes_le(radius_squared)
}

pub fn ray_aabb_time<const LANES: usize>(
    rays: [Simd<f32, LANES>; 6],
    aabb: [Simd<f32, LANES>; 6],
) -> Simd<f32, LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    let aabb_min = &aabb[0..3];
    let aabb_max = &aabb[3..6];
    let ray_origin = &rays[0..3];
    let ray_direction = &rays[3..6];

    let mut t_min_part1: [Simd<f32, LANES>; 3] = Default::default();
    let mut t_max_part1: [Simd<f32, LANES>; 3] = Default::default();
    for i in 0..3 {
        let t1 = (aabb_min[i] - ray_origin[i]) / ray_direction[i];
        let t2 = (aabb_max[i] - ray_origin[i]) / ray_direction[i];
        t_min_part1[i] = t1.min(t2);
        t_max_part1[i] = t1.max(t2);
    }

    let mut t_min: Simd<f32, LANES> = Default::default();
    let mut t_max: Simd<f32, LANES> = Default::default();
    for i in 0..LANES {
        let t_min_part2 = [t_min_part1[0][i], t_min_part1[1][i], t_min_part1[2][i]]
            .into_iter()
            .map(|item| FloatOrd(item))
            .max()
            .map(|item| item.0)
            .unwrap_or(0.0);
        let t_max_part2 = [t_max_part1[0][i], t_max_part1[1][i], t_max_part1[2][i]]
            .into_iter()
            .map(|item| FloatOrd(item))
            .min()
            .map(|item| item.0)
            .unwrap_or(0.0);

        t_min[i] = t_min_part2;
        t_max[i] = t_max_part2;
    }

    let zero = Simd::splat(0.0);
    let neg_one = Simd::splat(-1.0);

    let min_max_mask = t_min.lanes_le(t_max);
    let t_min_zero_mask = t_min.lanes_gt(zero);
    let t_max_zero_mask = t_max.lanes_gt(zero);
    min_max_mask.select(
        t_min_zero_mask.select(t_min, t_max_zero_mask.select(t_max, neg_one)),
        neg_one,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intersections;
    use crate::ray::Ray;
    use crate::shapes::Aabb;
    use crate::test_utils::generate_test_data;
    use crate::utils::simd_permutations;
    use rand::Rng;
    use std::ops::Range;

    #[test]
    fn aabb_aabb_intersections() {
        const LANES: usize = 8;
        const RANGE: Range<f32> = -2.0..2.0;
        const COUNT: usize = 100;
        let mut rng = rand::thread_rng();
        let (simd_aabbs, _) = generate_test_data(COUNT, |_| {
            [0.0, 0.0, 0.0, 1.0, 1.0, 1.0].map(|v| v + rng.gen_range(RANGE))
        });

        let mut has_non_intersection = false;
        let mut has_intersection = false;

        simd_permutations::<LANES, 6, 6, _>(&simd_aabbs, &simd_aabbs, |a, b| {
            let r = aabb_aabb::<LANES>(a, b).to_array();
            let mut r2 = [false; LANES];
            let mut a_min = [0.0; 3];
            let mut a_max = [0.0; 3];
            let mut b_min = [0.0; 3];
            let mut b_max = [0.0; 3];
            for i in 0..LANES {
                a_min[0] = a[0][i];
                a_min[1] = a[1][i];
                a_min[2] = a[2][i];
                a_max[0] = a[3][i];
                a_max[1] = a[4][i];
                a_max[2] = a[5][i];
                b_min[0] = b[0][i];
                b_min[1] = b[1][i];
                b_min[2] = b[2][i];
                b_max[0] = b[3][i];
                b_max[1] = b[4][i];
                b_max[2] = b[5][i];
                let aabb_a = Aabb {
                    min: a_min,
                    max: a_max,
                };
                let aabb_b = Aabb {
                    min: b_min,
                    max: b_max,
                };
                let i_r = intersections::aabb_aabb(aabb_a, aabb_b);
                if i_r {
                    has_intersection = true;
                } else {
                    has_non_intersection = true;
                }
                r2[i] = i_r;
            }
            assert!(r == r2, "SIMD results do not match non-SIMD results");
        });

        assert!(has_intersection, "No intersections found");
        assert!(has_non_intersection, "Only intersections found");
    }

    #[test]
    fn ray_aabb_intersections() {
        const LANES: usize = 8;
        const RANGE: Range<f32> = -2.0..2.0;
        const COUNT: usize = 100;
        let mut rng = rand::thread_rng();
        let (aabbs, _) = generate_test_data(COUNT, |_| {
            [0.0, 0.0, 0.0, 1.0, 1.0, 1.0].map(|v| v + rng.gen_range(RANGE))
        });
        let (rays, _) = generate_test_data(COUNT, |_| {
            [0.0, 0.0, 0.0, 1.0, 1.0, 1.0].map(|v| v + rng.gen_range(RANGE))
        });

        let mut has_non_intersection = false;
        let mut has_intersection = false;

        simd_permutations::<LANES, 6, 6, _>(&rays, &aabbs, |a, b| {
            let r = ray_aabb_time::<LANES>(a, b).to_array();
            let mut r2 = [0.0; LANES];
            let mut a_min = [0.0; 3];
            let mut a_max = [0.0; 3];
            let mut ray_origin = [0.0; 3];
            let mut ray_direction = [0.0; 3];
            for i in 0..LANES {
                ray_origin[0] = a[0][i];
                ray_origin[1] = a[1][i];
                ray_origin[2] = a[2][i];
                ray_direction[0] = a[3][i];
                ray_direction[1] = a[4][i];
                ray_direction[2] = a[5][i];
                a_min[0] = b[0][i];
                a_min[1] = b[1][i];
                a_min[2] = b[2][i];
                a_max[0] = b[3][i];
                a_max[1] = b[4][i];
                a_max[2] = b[5][i];
                let aabb = Aabb {
                    min: a_min,
                    max: a_max,
                };
                let ray = Ray {
                    origin: ray_origin,
                    direction: ray_direction,
                };
                let i_r = intersections::ray_aabb_time(ray, aabb);
                if i_r >= 0.0 {
                    has_intersection = true;
                } else {
                    has_non_intersection = true;
                }
                r2[i] = i_r;
            }
            assert!(r == r2, "SIMD results do not match non-SIMD results");
        });

        assert!(has_intersection, "No intersections found");
        assert!(has_non_intersection, "Only intersections found");
    }
}
