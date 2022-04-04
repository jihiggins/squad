use crate::ray::Ray;
use crate::shapes::*;
use crate::Vec3;
use float_ord::FloatOrd;

pub fn aabb_aabb(a: Aabb, b: Aabb) -> bool {
    a.min.zip(b.max).iter().all(|(a, b)| a <= b)
        && b.min.zip(a.max).iter().all(|(a, b)| a <= b)
}

pub fn aabb_point(aabb: Aabb, point: impl Into<Vec3>) -> bool {
    let point: [f32; 3] = point.into().into();
    aabb.min.zip(point).iter().all(|(a, b)| a <= b)
        && aabb.max.zip(point).iter().all(|(a, b)| a >= b)
}

pub fn aabb_sphere(aabb: Aabb, sphere: Sphere) -> bool {
    let sphere_center = sphere.center.zip(aabb.min).map(|(s_c, a_min)| s_c - a_min);
    let sphere_radius = sphere.radius;
    sphere_center
        .zip(aabb.max.zip(aabb.min))
        .iter()
        .all(|(s_c, (a_max, a_min))| s_c.abs() <= a_max - a_min + sphere_radius)
}

pub fn sphere_sphere(a: Sphere, b: Sphere) -> bool {
    let distance_squared = distance_squared(a.center, b.center);
    let total_radius = a.radius + b.radius;
    distance_squared <= total_radius * total_radius
}

pub fn sphere_point(sphere: Sphere, point: impl Into<Vec3>) -> bool {
    let point: [f32; 3] = point.into().into();
    let distance_squared = distance_squared(sphere.center, point);
    distance_squared <= sphere.radius * sphere.radius
}

pub fn distance_squared(a: [f32; 3], b: [f32; 3]) -> f32 {
    a.zip(b).iter().map(|(a, b)| (a - b).powi(2)).sum()
}

pub fn ray_aabb_time(ray: Ray, aabb: Aabb) -> f32 {
    let t1 = aabb.min.zip(ray.origin.zip(ray.direction)).map(
        |(aabb_min, (ray_origin, ray_direction))| {
            (aabb_min - ray_origin) / ray_direction
        },
    );
    let t2 = aabb.max.zip(ray.origin.zip(ray.direction)).map(
        |(aabb_max, (ray_origin, ray_direction))| {
            (aabb_max - ray_origin) / ray_direction
        },
    );
    let t_min = t1
        .zip(t2)
        .iter()
        .map(|(t1, t2)| FloatOrd(t1.min(*t2)))
        .max()
        .map(|f| f.0)
        .unwrap_or(0.0);
    let t_max = t1
        .zip(t2)
        .iter()
        .map(|(t1, t2)| FloatOrd(t1.max(*t2)))
        .min()
        .map(|f| f.0)
        .unwrap_or(0.0);
    if t_min <= t_max {
        if t_min > 0.0 {
            t_min
        } else if t_max > 0.0 {
            t_max
        } else {
            -1.0
        }
    } else {
        -1.0
    }
}

pub fn ray_aabb(ray: Ray, aabb: Aabb) -> bool {
    ray_aabb_time(ray, aabb) >= 0.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aabb_aabb_intersections() {
        let aabb1 = Aabb::new([0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);
        let aabb2 = Aabb::new([0.5, 0.5, 0.5], [1.5, 1.5, 1.5]);
        assert!(aabb_aabb(aabb1, aabb2));
        let aabb3 = Aabb::new([1.2, 1.2, 1.2], [2.0, 2.0, 2.0]);
        assert!(!aabb_aabb(aabb1, aabb3));
    }

    #[test]
    fn aabb_point_intersections() {
        let aabb1 = Aabb::new([0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);
        assert!(aabb_point(aabb1, [0.5, 0.5, 0.5]));
        assert!(!aabb_point(aabb1, [1.5, 1.5, 1.5]));
    }

    #[test]
    fn aabb_sphere_intersections() {
        let aabb1 = Aabb::new([0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);
        let sphere1 = Sphere::new([0.5, 0.5, 0.5], 0.5);
        assert!(aabb_sphere(aabb1, sphere1));
        let sphere2 = Sphere::new([1.6, 1.5, 1.5], 0.5);
        assert!(!aabb_sphere(aabb1, sphere2));
    }

    #[test]
    fn sphere_sphere_intersections() {
        let sphere1 = Sphere::new([0.0, 0.0, 0.0], 0.5);
        let sphere2 = Sphere::new([0.5, 0.5, 0.5], 0.5);
        assert!(sphere_sphere(sphere1, sphere2));
        let sphere3 = Sphere::new([1.5, 1.0, 1.0], 0.5);
        assert!(!sphere_sphere(sphere1, sphere3));
    }

    #[test]
    fn sphere_point_intersections() {
        let sphere1 = Sphere::new([0.0, 0.0, 0.0], 1.0);
        assert!(sphere_point(sphere1, [0.5, 0.5, 0.5]));
        assert!(!sphere_point(sphere1, [1.5, 1.5, 1.5]));
    }

    #[test]
    fn ray_aabb_intersections() {
        let ray1 = Ray::new([0.0, -5.0, 0.0], [0.0, 1.0, 0.0]);
        let aabb1 = Aabb::new([-1.0, -1.0, -1.0], [2.0, 2.0, 2.0]);
        assert!(ray_aabb(ray1, aabb1));
        let ray2 = Ray::new([0.0, -5.0, 0.0], [0.0, 1.0, 0.0]);
        let aabb2 = Aabb::new([1.0, 1.0, 1.0], [2.0, 2.0, 2.0]);
        assert!(!ray_aabb(ray2, aabb2));
    }
}
