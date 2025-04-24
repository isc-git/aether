use crate::ray;
use crate::shapes;

pub struct Object<T: shapes::Traceable> {
    color: [u8; 3],
    shape: T,
}

impl<T: shapes::Traceable> Object<T> {
    pub fn new(shape: T, color: [u8; 3]) -> Self {
        Self { color, shape }
    }

    pub fn closest_ray_intersection(&self, ray: &ray::Ray, epsilon: f32) -> Option<f32> {
        self.shape.trace(ray, epsilon)
    }

    pub fn color(&self) -> [u8; 3] {
        self.color
    }
}
