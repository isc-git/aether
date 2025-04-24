use crate::ray;

pub mod composite;
pub mod plane;
pub mod sphere;
pub mod triangle;

pub trait Traceable {
    /// trace to the first intersection in a positive direction
    fn trace(&self, ray: &ray::Ray, epsilon: f32) -> Option<f32>;
}

pub enum Shape {
    Composite(composite::CompositeObject),
    Plane(plane::Plane),
    Sphere(sphere::Sphere),
    Triangle(triangle::Triangle),
}

impl Traceable for Shape {
    fn trace(&self, ray: &ray::Ray, epsilon: f32) -> Option<f32> {
        match self {
            Shape::Composite(s) => s.trace(ray, epsilon),
            Shape::Plane(s) => s.trace(ray, epsilon),
            Shape::Sphere(s) => s.trace(ray, epsilon),
            Shape::Triangle(s) => s.trace(ray, epsilon),
        }
    }
}
