use angle::Angle;
use ray::Ray;
use shapes::triangle::Triangle;

pub mod angle;
pub mod camera;
pub mod distance;
pub mod object;
pub mod ray;
pub mod shapes;

pub fn ray_mesh_intersections(
    mesh: &[Triangle],
    ray: &Ray,
    epsilon: f32,
) -> impl Iterator<Item = f32> {
    mesh.iter()
        .filter_map(move |triangle| triangle.ray_intersection(ray, epsilon))
}

#[inline(always)]
pub(crate) fn is_zero(float: f32, epsilon: f32) -> bool {
    debug_assert!(epsilon > 0.0, "epsilon can not be negative");
    float.abs() <= epsilon
}

pub fn reflection(
    direction: &nalgebra::Vector3<f32>,
    norm: &nalgebra::UnitVector3<f32>,
) -> nalgebra::Vector3<f32> {
    let perpendicular_component = direction.dot(norm) * norm.into_inner();
    direction - 2. * perpendicular_component
}

/// Assuming non-magnetic materials
///
/// # Arguments
/// - `incident_angle`: angle of incidence of light, from plane to ray
/// - `transmitted_angle`: angle of transmitted light, from plane to ray
pub fn reflectance_s_polarized(
    incident_angle: Angle,
    transmitted_angle: Angle,
    incident_refractive_index: f32,
    transmitted_refractive_index: f32,
) -> f32 {
    let a = incident_refractive_index * incident_angle.cos();
    let b = transmitted_refractive_index * transmitted_angle.cos();

    ((a - b) / (a + b)).powi(2)
}

/// Assuming non-magnetic materials
///
/// # Arguments
/// - `incident_angle`: angle of incidence of light, from plane to ray
/// - `transmitted_angle`: angle of transmitted light, from plane to ray
pub fn reflectance_p_polarized(
    incident_angle: Angle,
    transmitted_angle: Angle,
    incident_refractive_index: f32,
    transmitted_refractive_index: f32,
) -> f32 {
    let a = incident_refractive_index * transmitted_angle.cos();
    let b = transmitted_refractive_index * incident_angle.cos();

    ((a - b) / (a + b)).powi(2)
}
