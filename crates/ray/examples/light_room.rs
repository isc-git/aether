use std::f32::consts::PI;

use ray::{camera, distance, shapes::triangle};

const CAMERA_WIDTH: u32 = 1920;
const CAMERA_HEIGHT: u32 = 1080;
const CAMERA_PIXEL_PITCH: distance::Distance = distance::Distance::from_um(2.);
const CAMERA_FOCAL_LENGTH: distance::Distance = distance::Distance::from_mm(30.);

const ROOM_HEIGHT: distance::Distance = distance::Distance::from_m(10.);
const ROOM_WIDTH: distance::Distance = distance::Distance::from_m(10.);
const ROOM_DEPTH: distance::Distance = distance::Distance::from_m(100.);

fn main() {
    let camera = camera::Camera::new(
        CAMERA_WIDTH,
        CAMERA_HEIGHT,
        CAMERA_PIXEL_PITCH,
        CAMERA_FOCAL_LENGTH,
    );
    let camera_orientation_ned = nalgebra::Rotation3::from_euler_angles(0., 0., 0.);
    let camera_to_body =
        nalgebra::Rotation3::from_euler_angles(90f32.to_radians(), 0., 90f32.to_radians());
    let camera_to_ned = camera_orientation_ned * camera_to_body;

    let hfov = camera.hfov();
    let vfov = camera.vfov();

    println!("hfov: {}", hfov);
    println!("vfov: {}", vfov);

    // determine how far back we should be to view the room
    let distance_for_horizontal = (ROOM_WIDTH.m() * 0.5) / (hfov * 0.5).tan();
    let distance_for_vertical = (ROOM_HEIGHT.m() * 0.5) / (vfov * 0.5).tan();
    let stepback_distance = distance_for_vertical.max(distance_for_horizontal);

    println!(
        "step back distance: max({}, {}): {}",
        distance_for_horizontal, distance_for_vertical, stepback_distance
    );

    // calculate rays out of camera
    let camera_position_ned = nalgebra::Vector3::new(-stepback_distance, 0., -ROOM_HEIGHT.m() / 2.);
    let camera_rays = (0..camera.width_px() * camera.height_px())
        .map(|i| {
            let row = i / camera.width_px();
            let col = i % camera.width_px();
            nalgebra::UnitVector3::new_normalize(
                camera_to_ned * camera.pixel_to_camera_vector(col as f32, row as f32),
            )
        })
        .collect::<Vec<_>>();

    let walls = [
        (
            nalgebra::Vector3::new(0., 0., -ROOM_HEIGHT.m()),
            nalgebra::Vector3::new(0., 0., 1.),
        ),
        (
            nalgebra::Vector3::new(0., -ROOM_WIDTH.m() / 2., 0.),
            nalgebra::Vector3::new(0., 1., 0.),
        ),
        (
            nalgebra::Vector3::new(0., 0., 0.),
            nalgebra::Vector3::new(0., 0., -1.),
        ),
        (
            nalgebra::Vector3::new(0., ROOM_WIDTH.m() / 2., 0.),
            nalgebra::Vector3::new(0., -1., 0.),
        ),
        (
            nalgebra::Vector3::new(ROOM_DEPTH.m(), 0., 0.),
            nalgebra::Vector3::new(-1., 0., 0.),
        ),
    ]
    .iter()
    .map(|(origin, normal)| {
        let plane =
            ray::shapes::plane::Plane::new(*origin, nalgebra::UnitVector3::new_normalize(*normal));
        ray::object::Object::new(ray::shapes::Shape::Plane(plane), [255, 255, 255])
    })
    .collect::<Vec<_>>();

    let mut unit_cube = ray::shapes::composite::CompositeObject::new(
        [
            // bottom
            triangle::quad_to_triangles(
                nalgebra::Vector3::new(0., 0., 0.),
                nalgebra::Vector3::new(1., 0., 0.),
                nalgebra::Vector3::new(1., 1., 0.),
                nalgebra::Vector3::new(0., 1., 0.),
            ),
            // back
            triangle::quad_to_triangles(
                nalgebra::Vector3::new(1., 0., 0.),
                nalgebra::Vector3::new(1., 0., 1.),
                nalgebra::Vector3::new(1., 1., 1.),
                nalgebra::Vector3::new(1., 1., 0.),
            ),
            // top
            triangle::quad_to_triangles(
                nalgebra::Vector3::new(0., 0., 1.),
                nalgebra::Vector3::new(0., 1., 1.),
                nalgebra::Vector3::new(1., 1., 1.),
                nalgebra::Vector3::new(1., 0., 1.),
            ),
            // front
            triangle::quad_to_triangles(
                nalgebra::Vector3::new(0., 0., 0.),
                nalgebra::Vector3::new(0., 0., 1.),
                nalgebra::Vector3::new(0., 1., 1.),
                nalgebra::Vector3::new(0., 1., 0.),
            ),
            // right
            triangle::quad_to_triangles(
                nalgebra::Vector3::new(1., 1., 0.),
                nalgebra::Vector3::new(1., 1., 1.),
                nalgebra::Vector3::new(0., 1., 1.),
                nalgebra::Vector3::new(0., 1., 0.),
            ),
            // left
            triangle::quad_to_triangles(
                nalgebra::Vector3::new(1., 0., 0.),
                nalgebra::Vector3::new(1., 0., 1.),
                nalgebra::Vector3::new(0., 0., 1.),
                nalgebra::Vector3::new(0., 0., 0.),
            ),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>(),
        [255, 0, 0],
        1.,
        0.,
    );

    unit_cube.translate_in_place(&nalgebra::Vector3::new(-0.5, -0.5, -0.5));
    let rotation = nalgebra::UnitQuaternion::from_axis_angle(
        &nalgebra::UnitVector3::new_normalize(nalgebra::Vector3::new(-1., 0., -1.)),
        PI / 2.,
    );
    unit_cube.rotate_in_place(&rotation);
    unit_cube.translate_in_place(&nalgebra::Vector3::new(0., 0., -ROOM_HEIGHT.m() / 2.));

    let intersections = camera_rays
        .iter()
        .map(|ray| {
            let camera_ray = ray::ray::Ray::new(camera_position_ned, *ray);
            walls
                .iter()
                .filter_map(|shape| {
                    shape
                        .closest_ray_intersection(&camera_ray, 1e-6)
                        .map(|dist| (dist, shape.color()))
                })
                .min_by(|a, b| a.0.total_cmp(&b.0))
        })
        .collect::<Vec<_>>();

    let box_intersections = camera_rays.iter().map(|ray| {
        let camera_ray = ray::ray::Ray::new(camera_position_ned, *ray);
        unit_cube
            .ray_intersection(&camera_ray, 1e-6)
            .filter(|a| *a > 0.)
            .min_by(|a, b| a.total_cmp(b))
            .map(|dist| (dist, *unit_cube.color()))
    });

    let intersections = box_intersections
        .zip(intersections.iter())
        .map(|(mbox, wall)| match (mbox, wall) {
            (None, None) => None,
            (Some(a), None) => Some(a),
            (None, Some(a)) => Some(a).copied(),
            (Some(a), Some(b)) => {
                if a.0 < b.0 {
                    Some(a)
                } else {
                    Some(b).copied()
                }
            }
        })
        .collect::<Vec<_>>();

    // ---- plot results ----

    let (max, min) =
        intersections.iter().fold(
            (f32::MIN, f32::MAX),
            |(max, min), distance| match distance {
                Some((distance, _)) => (distance.max(max), distance.min(min)),
                None => (max, min),
            },
        );

    let mut distance_map = image::RgbImage::new(camera.width_px(), camera.height_px());
    distance_map.pixels_mut().enumerate().for_each(|(i, rgb)| {
        if let Some((distance, _)) = intersections[i] {
            let value = ((1.0 - ((distance - min) / (max - min))).powf(0.6) * 255. * 3.) as u16;
            let r = value.min(255);
            let value = value.saturating_sub(255);
            let g = value.min(255);
            let value = value.saturating_sub(255);
            let b = value;

            rgb[0] = r as u8;
            rgb[1] = g as u8;
            rgb[2] = b as u8;
        }
    });

    distance_map
        .save("room_depth.tiff")
        .expect("able to save distance_map");

    let mut color_map = image::RgbImage::new(camera.width_px(), camera.height_px());
    color_map.pixels_mut().enumerate().for_each(|(i, rgb)| {
        if let Some((_, color)) = intersections[i] {
            rgb[0] = color[0];
            rgb[1] = color[1];
            rgb[2] = color[2];
        }
    });

    color_map
        .save("room_color.tiff")
        .expect("able to save color_map");
}
