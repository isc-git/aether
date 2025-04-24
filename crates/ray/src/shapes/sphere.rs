use crate::ray;

pub enum SphereIntersection {
    Two(f32, f32),
    One(f32),
    None,
}

pub struct Sphere {
    origin: nalgebra::Vector3<f32>,
    radius: f32,
}

impl Sphere {
    pub fn new(origin: nalgebra::Vector3<f32>, radius: f32) -> Self {
        Self { origin, radius }
    }

    pub fn ray_intersection(&self, ray: &ray::Ray, epsilon: f32) -> SphereIntersection {
        debug_assert!(epsilon > 0.0, "epsilon can not be negative");

        let center_dist = ray.origin() - self.origin;
        let dir_dot_dist = ray.direction().dot(&center_dist);

        let discriminant =
            dir_dot_dist.powi(2) - (center_dist.dot(&center_dist) - self.radius.powi(2));

        if discriminant < 0. {
            return SphereIntersection::None;
        }

        if crate::is_zero(discriminant, epsilon) {
            return SphereIntersection::One(-dir_dot_dist);
        }

        let disc_sqrt = discriminant.sqrt();
        SphereIntersection::Two(-dir_dot_dist + disc_sqrt, -dir_dot_dist - disc_sqrt)
    }
}

impl super::Traceable for Sphere {
    fn trace(&self, ray: &ray::Ray, epsilon: f32) -> Option<f32> {
        match self.ray_intersection(ray, epsilon) {
            SphereIntersection::Two(a, b) => match (a >= 0., b >= 0.) {
                (true, true) => Some(a.min(b)),
                (true, false) => Some(a),
                (false, true) => Some(b),
                (false, false) => None,
            },
            SphereIntersection::One(a) if a >= 0. => Some(a),
            _ => None,
        }
    }
}
