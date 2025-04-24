use crate::ray;

pub enum PlaneIntersection {
    Intersection(f32),
    Contained,
    None,
}

pub struct Plane {
    origin: nalgebra::Vector3<f32>,
    normal: nalgebra::UnitVector3<f32>,
}

impl Plane {
    pub fn new(origin: nalgebra::Vector3<f32>, normal: nalgebra::UnitVector3<f32>) -> Self {
        Self { origin, normal }
    }

    pub fn ray_intersection(&self, ray: &ray::Ray, epsilon: f32) -> PlaneIntersection {
        debug_assert!(epsilon > 0.0, "epsilon can not be negative");

        let denominator = ray.direction().dot(&self.normal);
        let numerator = (self.origin - ray.origin()).dot(&self.normal);

        match (
            crate::is_zero(numerator, epsilon),
            crate::is_zero(denominator, epsilon),
        ) {
            (true, true) => PlaneIntersection::Contained,
            (false, true) => PlaneIntersection::None,
            _ => PlaneIntersection::Intersection(numerator / denominator),
        }
    }
}

impl super::Traceable for Plane {
    fn trace(&self, ray: &crate::ray::Ray, epsilon: f32) -> Option<f32> {
        match self.ray_intersection(ray, epsilon) {
            PlaneIntersection::Intersection(dist) if dist >= 0. => Some(dist),
            PlaneIntersection::Contained => Some(0.),
            _ => None,
        }
    }
}
