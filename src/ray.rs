use crate::Vec3;
use std::ops::Range;

#[derive(Debug, Clone, Copy, Default)]
pub struct Ray {
    pub origin: [f32; 3],
    pub direction: [f32; 3],
}

impl Ray {
    pub fn new(origin: impl Into<Vec3>, direction: impl Into<Vec3>) -> Self {
        let origin = origin.into().into();
        let direction = direction.into().into();
        Self { origin, direction }
    }

    pub fn get_point<T: std::convert::From<[f32; 3]>>(&self, t: f32) -> T {
        (self
            .origin
            .zip(self.direction.map(|d| d * t))
            .map(|(a, b)| a + b))
        .into()
    }

    pub fn get_range<T: std::convert::From<[f32; 3]>>(
        &self,
        r: Range<f32>,
        step: f32,
    ) -> impl Iterator<Item = T> + '_ {
        let mut t = r.start;
        std::iter::from_fn(move || {
            let point = self.get_point(t);
            t += step;
            if t > r.end {
                None
            } else {
                Some(point)
            }
        })
    }
}
