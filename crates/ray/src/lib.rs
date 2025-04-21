use nalgebra::Vector3;

pub struct Object {
    color: [u8; 3],
    shape: Shape,
}

impl Object {
    pub fn new(shape: Shape, color: [u8; 3]) -> Self {
        Self { color, shape }
    }

    pub fn closest_ray_intersection(&self, ray: &Ray, epsilon: f32) -> Option<f32> {
        match &self.shape {
            Shape::Plane(plane) => match ray_plane_intersection(ray, plane, epsilon) {
                RayIntersection::Intersection(dist) => Some(dist),
                RayIntersection::Contained => Some(0.),
                RayIntersection::None => None,
            },
            Shape::Sphere(sphere) => match ray_sphere_intersection(ray, sphere, epsilon) {
                SphereIntersection::Two(a, b) => Some(a.min(b)),
                SphereIntersection::One(dist) => Some(dist),
                SphereIntersection::None => None,
            },
        }
    }

    pub fn color(&self) -> [u8; 3] {
        self.color
    }
}

pub enum Shape {
    Plane(Plane),
    Sphere(Sphere),
}

pub struct Ray {
    origin: nalgebra::Vector3<f32>,
    // assume normalized
    direction: nalgebra::UnitVector3<f32>,
}

impl Ray {
    pub fn new(origin: nalgebra::Vector3<f32>, direction: nalgebra::UnitVector3<f32>) -> Self {
        Self { origin, direction }
    }
}

pub struct Sphere {
    origin: nalgebra::Vector3<f32>,
    radius: f32,
}

impl Sphere {
    pub fn new(origin: nalgebra::Vector3<f32>, radius: f32) -> Self {
        Self { origin, radius }
    }
}

pub enum SphereIntersection {
    Two(f32, f32),
    One(f32),
    None,
}

pub enum RayIntersection {
    Intersection(f32),
    Contained,
    None,
}

#[inline(always)]
fn is_zero(float: f32, epsilon: f32) -> bool {
    debug_assert!(epsilon > 0.0, "epsilon can not be negative");
    float.abs() <= epsilon
}

pub struct Plane {
    origin: nalgebra::Vector3<f32>,
    normal: nalgebra::UnitVector3<f32>,
}

impl Plane {
    pub fn new(origin: nalgebra::Vector3<f32>, normal: nalgebra::UnitVector3<f32>) -> Self {
        Self { origin, normal }
    }
}

pub fn ray_plane_intersection(ray: &Ray, plane: &Plane, epsilon: f32) -> RayIntersection {
    debug_assert!(epsilon > 0.0, "epsilon can not be negative");

    let denominator = ray.direction.dot(&plane.normal);
    let numerator = (plane.origin - ray.origin).dot(&plane.normal);

    match (is_zero(numerator, epsilon), is_zero(denominator, epsilon)) {
        (true, true) => RayIntersection::Contained,
        (false, true) => RayIntersection::None,
        _ => RayIntersection::Intersection(numerator / denominator),
    }
}

pub fn ray_sphere_intersection(ray: &Ray, sphere: &Sphere, epsilon: f32) -> SphereIntersection {
    debug_assert!(epsilon > 0.0, "epsilon can not be negative");

    let center_dist = ray.origin - sphere.origin;
    let dir_dot_dist = ray.direction.dot(&center_dist);

    let discriminant =
        dir_dot_dist.powi(2) - (center_dist.dot(&center_dist) - sphere.radius.powi(2));

    if discriminant < 0. {
        return SphereIntersection::None;
    }

    if is_zero(discriminant, epsilon) {
        return SphereIntersection::One(-dir_dot_dist);
    }

    let disc_sqrt = discriminant.sqrt();
    SphereIntersection::Two(-dir_dot_dist + disc_sqrt, -dir_dot_dist - disc_sqrt)
}

pub fn reflection(direction: &Vector3<f32>, norm: &nalgebra::UnitVector3<f32>) -> Vector3<f32> {
    let perpendicular_component = direction.dot(norm) * norm.into_inner();
    direction - 2. * perpendicular_component
}
