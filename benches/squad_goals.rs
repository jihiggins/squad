#![feature(portable_simd)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use parry3d::math::{Isometry, Real, Vector};
use parry3d::na::Vector3;
use rand::Rng;
use squad::ray::Ray;
use squad::shapes::*;
use squad::test_utils::generate_test_data;
use squad::utils::simd_permutations;
use squad::*;
use std::ops::Range;
use std::simd::Simd;

fn criterion_benchmark(c: &mut Criterion) {
    const RANGE: Range<f32> = -1000.0..1000.0;
    const AABB_COUNT: usize = 1000;
    const RAY_COUNT: usize = 100;
    const LANES: usize = 8;

    let mut rng = rand::thread_rng();

    let (simd_aabbs, aabbs) = generate_test_data(AABB_COUNT, |_| {
        [
            rng.gen_range(RANGE),
            rng.gen_range(RANGE),
            rng.gen_range(RANGE),
            rng.gen_range(RANGE),
            rng.gen_range(RANGE),
            rng.gen_range(RANGE),
        ]
    });

    let (simd_rays, rays) = generate_test_data(RAY_COUNT, |_| {
        [
            rng.gen_range(RANGE),
            rng.gen_range(RANGE),
            rng.gen_range(RANGE),
            rng.gen_range(RANGE),
            rng.gen_range(RANGE),
            rng.gen_range(RANGE),
        ]
    });

    let mut group = c.benchmark_group("Intersections");
    group.bench_function("aabb_aabb_intersections_simd", |b| {
        b.iter(|| {
            black_box(simd_permutations::<LANES, 6, 6, _>(
                &simd_aabbs,
                &simd_aabbs,
                |a, b| {
                    wide_intersections::aabb_aabb::<LANES>(a, b);
                },
            ))
        })
    });

    group.bench_function("aabb_aabb_intersections", |b| {
        b.iter(|| {
            for i in 0..AABB_COUNT {
                for j in (i + 1)..AABB_COUNT {
                    let a = aabbs[i];
                    let b = aabbs[j];
                    let a = Aabb {
                        min: a[0],
                        max: a[1],
                    };
                    let b = Aabb {
                        min: b[0],
                        max: b[1],
                    };
                    black_box(intersections::aabb_aabb(a, b));
                }
            }
        });
    });

    let mut parry_positions = vec![];
    let mut parry_cubiods = vec![];
    for i in 0..AABB_COUNT {
        let a = aabbs[i];
        let a_c = parry3d::shape::Cuboid::new(Vector::<Real>::new(
            (a[0][0] - a[1][0]).abs() / 2.0,
            (a[0][1] - a[1][1]).abs() / 2.0,
            (a[0][2] - a[1][2]).abs() / 2.0,
        ));
        let p1 = Isometry::<Real>::new(
            Vector3::<Real>::new(a[0][0], a[0][1], a[0][2]),
            Vector3::<Real>::new(0.0, 1.0, 0.0),
        );
        parry_positions.push(p1);
        parry_cubiods.push(a_c);
    }

    /*
    // TODO: This is probably not actually a good comparison
    group.bench_function("aabb_aabb_intersections_parry", |b| {
        b.iter(|| {
            for i in 0..AABB_COUNT {
                for j in (i + 1)..AABB_COUNT {
                    black_box(parry3d::query::intersection_test(
                        &parry_positions[i],
                        &parry_cubiods[i],
                        &parry_positions[j],
                        &parry_cubiods[j],
                    ));
                }
            }
        });
    });
    */

    group.bench_function("ray_aabb_intersections_simd", |b| {
        b.iter(|| {
            black_box(simd_permutations::<LANES, 6, 6, _>(
                &simd_rays,
                &simd_aabbs,
                |a, b| {
                    wide_intersections::ray_aabb_time::<LANES>(a, b);
                },
            ))
        })
    });

    group.bench_function("ray_aabb_intersections", |b| {
        b.iter(|| {
            for i in 0..RAY_COUNT {
                for j in 0..AABB_COUNT {
                    let a = rays[i];
                    let b = aabbs[j];
                    let a = Ray {
                        origin: a[0],
                        direction: a[1],
                    };
                    let b = Aabb {
                        min: b[0],
                        max: b[1],
                    };
                    black_box(intersections::ray_aabb_time(a, b));
                }
            }
        });
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
