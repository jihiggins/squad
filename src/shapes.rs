use crate::Vec3;
use treeculler::{BVol, Frustum};

#[derive(Debug, Clone, Copy)]
pub struct Aabb {
    pub min: [f32; 3],
    pub max: [f32; 3],
}

#[derive(Debug, Clone, Copy)]
pub struct Obb {
    pub center: [f32; 3],
    pub half_extents: [f32; 3],
    pub axes: [[f32; 3]; 3],
}

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub center: [f32; 3],
    pub radius: f32,
}

impl Aabb {
    pub fn new(min: impl Into<Vec3>, max: impl Into<Vec3>) -> Self {
        Self {
            min: min.into().into(),
            max: max.into().into(),
        }
    }

    pub fn with_offset(&self, offset: impl Into<Vec3>) -> Self {
        let offset = offset.into().into();
        Self {
            min: self.min.zip(offset).map(|(a, b)| a + b),
            max: self.max.zip(offset).map(|(a, b)| a + b),
        }
    }

    pub fn within_frustum(&self, frustum: &Frustum<f32>) -> bool {
        let aabb = treeculler::AABB::<f32>::new(self.min, self.max);
        aabb.coherent_test_against_frustum(frustum, 0).0
    }
}

impl Sphere {
    pub fn new(center: impl Into<Vec3>, radius: f32) -> Self {
        Self {
            center: center.into().into(),
            radius,
        }
    }
}
