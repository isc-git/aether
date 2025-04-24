use super::triangle;
use crate::ray;
use crate::ray_mesh_intersections;

pub struct CompositeObject {
    mesh: Vec<triangle::Triangle>,
    color: [u8; 3],
    refractive_index: f32,
    extinction_coefficient: f32,
}

impl CompositeObject {
    pub fn new(
        mesh: Vec<triangle::Triangle>,
        color: [u8; 3],
        refractive_index: f32,
        extinction_coefficient: f32,
    ) -> Self {
        Self {
            mesh,
            color,
            refractive_index,
            extinction_coefficient,
        }
    }

    pub fn ray_intersection(&self, ray: &ray::Ray, epsilon: f32) -> impl Iterator<Item = f32> {
        ray_mesh_intersections(&self.mesh, ray, epsilon)
    }

    pub fn color(&self) -> &[u8; 3] {
        &self.color
    }

    pub fn refractive_index(&self) -> f32 {
        self.refractive_index
    }

    pub fn extinction_coefficient(&self) -> f32 {
        self.extinction_coefficient
    }

    pub fn scale_in_place(&mut self, scale: f32) {
        self.mesh
            .iter_mut()
            .for_each(|triangle| *triangle = triangle.scale(scale))
    }

    pub fn rotate_in_place(&mut self, rotation: &nalgebra::UnitQuaternion<f32>) {
        self.mesh
            .iter_mut()
            .for_each(|triangle| *triangle = triangle.rotate(rotation))
    }

    /// make sure you are in the right coords
    pub fn translate_in_place(&mut self, translation: &nalgebra::Vector3<f32>) {
        self.mesh
            .iter_mut()
            .for_each(|triangle| *triangle = triangle.translate(translation))
    }
}

impl super::Traceable for CompositeObject {
    fn trace(&self, ray: &ray::Ray, epsilon: f32) -> Option<f32> {
        self.ray_intersection(ray, epsilon)
            .filter(|dist| *dist >= 0.)
            .min_by(|a, b| a.total_cmp(b))
    }
}
