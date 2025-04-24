use crate::ray;

pub struct Triangle {
    a: nalgebra::Vector3<f32>,
    b: nalgebra::Vector3<f32>,
    c: nalgebra::Vector3<f32>,
}

impl Triangle {
    pub fn new(
        a: nalgebra::Vector3<f32>,
        b: nalgebra::Vector3<f32>,
        c: nalgebra::Vector3<f32>,
    ) -> Self {
        Self { a, b, c }
    }

    pub fn rotate(&self, rotation: &nalgebra::UnitQuaternion<f32>) -> Self {
        Self::new(
            rotation.transform_vector(&self.a),
            rotation.transform_vector(&self.b),
            rotation.transform_vector(&self.c),
        )
    }

    pub fn translate(&self, translation: &nalgebra::Vector3<f32>) -> Self {
        Self::new(
            self.a + translation,
            self.b + translation,
            self.c + translation,
        )
    }

    /// scales away from origin
    pub fn scale(&self, scale: f32) -> Self {
        Self::new(scale * self.a, scale * self.b, scale * self.c)
    }

    pub fn a(&self) -> &nalgebra::Vector3<f32> {
        &self.a
    }

    pub fn b(&self) -> &nalgebra::Vector3<f32> {
        &self.b
    }

    pub fn c(&self) -> &nalgebra::Vector3<f32> {
        &self.c
    }

    /// taken from https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm
    pub fn ray_intersection(&self, ray: &ray::Ray, epsilon: f32) -> Option<f32> {
        let edge1 = self.b() - self.a();
        let edge2 = self.c() - self.a();

        let ray_cross_edge2 = ray.direction().cross(&edge2);
        let det = edge1.dot(&ray_cross_edge2);

        if crate::is_zero(det, epsilon) {
            return None;
        }

        let inv_det = 1.0 / det;
        let s = ray.origin() - self.a();
        let u = inv_det * s.dot(&ray_cross_edge2);
        if !(0.0..=1.0).contains(&u) {
            return None;
        }

        let s_cross_e1 = s.cross(&edge1);
        let v = inv_det * ray.direction().dot(&s_cross_e1);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = inv_det * edge2.dot(&s_cross_e1);

        if t < 0. { None } else { Some(t) }
    }
}

impl super::Traceable for Triangle {
    fn trace(&self, ray: &ray::Ray, epsilon: f32) -> Option<f32> {
        match self.ray_intersection(ray, epsilon) {
            Some(dist) if dist >= 0. => Some(dist),
            _ => None,
        }
    }
}

pub fn quad_to_triangles(
    a: nalgebra::Vector3<f32>,
    b: nalgebra::Vector3<f32>,
    c: nalgebra::Vector3<f32>,
    d: nalgebra::Vector3<f32>,
) -> [Triangle; 2] {
    [Triangle::new(a, b, c), Triangle::new(c, d, a)]
}
