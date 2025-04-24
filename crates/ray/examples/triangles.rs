use std::f32::consts::PI;

use ray::shapes::triangle;

fn main() {
    let camera_location = nalgebra::Vector3::new(-500., 0., 0.);
    let camera_orientation_ned = nalgebra::Rotation3::from_euler_angles(0., 0., 0.);
    let camera = ray::camera::Camera::new(
        101,
        101,
        ray::distance::Distance::from_um(2.0),
        ray::distance::Distance::from_m(0.03),
    );

    // make a square
    // no care was taken w.r.t winding direction
    //
    // lets say we are making these in world coordinates
    // - x: north
    // - y: east
    // - z: down
    let input_triangles = [
        // front face
        triangle::quad_to_triangles(
            nalgebra::Vector3::new(-0.5, 0.5, -0.5),
            nalgebra::Vector3::new(0.5, 0.5, -0.5),
            nalgebra::Vector3::new(0.5, -0.5, -0.5),
            nalgebra::Vector3::new(-0.5, -0.5, -0.5),
        ),
        // right face
        triangle::quad_to_triangles(
            nalgebra::Vector3::new(0.5, 0.5, -0.5),
            nalgebra::Vector3::new(0.5, 0.5, 0.5),
            nalgebra::Vector3::new(0.5, -0.5, 0.5),
            nalgebra::Vector3::new(0.5, -0.5, -0.5),
        ),
        // back face
        triangle::quad_to_triangles(
            nalgebra::Vector3::new(0.5, 0.5, 0.5),
            nalgebra::Vector3::new(-0.5, 0.5, 0.5),
            nalgebra::Vector3::new(-0.5, -0.5, 0.5),
            nalgebra::Vector3::new(0.5, -0.5, 0.5),
        ),
        // left face
        triangle::quad_to_triangles(
            nalgebra::Vector3::new(-0.5, 0.5, -0.5),
            nalgebra::Vector3::new(-0.5, 0.5, 0.5),
            nalgebra::Vector3::new(-0.5, -0.5, 0.5),
            nalgebra::Vector3::new(-0.5, -0.5, -0.5),
        ),
    ];

    let rotation_axis = nalgebra::UnitVector3::new_normalize(nalgebra::Vector3::new(-1., 0., -1.));
    let rotation = nalgebra::UnitQuaternion::from_axis_angle(&rotation_axis, PI / 2.);
    let triangles = input_triangles
        .iter()
        .flatten()
        .map(|t| t.rotate(&rotation))
        .collect::<Vec<_>>();

    // transforms from camera vec to ned
    let camera_to_body =
        nalgebra::Rotation3::from_euler_angles(90f32.to_radians(), 0., 90f32.to_radians());
    let camera_to_ned = camera_orientation_ned * camera_to_body;

    let camera_rays = (0..camera.width_px() * camera.height_px())
        .map(|i| {
            let row = i / camera.width_px();
            let col = i % camera.width_px();
            nalgebra::UnitVector3::new_normalize(
                camera_to_ned * camera.pixel_to_camera_vector(col as f32, row as f32),
            )
        })
        .collect::<Vec<_>>();

    let intersections = camera_rays
        .iter()
        .map(|ray| {
            let camera_ray = ray::ray::Ray::new(camera_location, *ray);
            triangles
                .iter()
                .filter_map(|shape| shape.ray_intersection(&camera_ray, 0.0001))
                .min_by(|a, b| a.total_cmp(b))
        })
        .collect::<Vec<_>>();

    let (max, min) =
        intersections.iter().fold(
            (f32::MIN, f32::MAX),
            |(max, min), distance| match distance {
                Some(distance) => (distance.max(max), distance.min(min)),
                None => (max, min),
            },
        );
    println!("{} -> {}", min, max);

    let mut distance_map = image::RgbImage::new(camera.width_px(), camera.height_px());
    distance_map.pixels_mut().enumerate().for_each(|(i, rgb)| {
        if let Some(distance) = intersections[i] {
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
        .save("triangle.tiff")
        .expect("able to save distance_map");
}
