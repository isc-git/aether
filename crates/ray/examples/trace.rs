fn main() {
    let camera_location = nalgebra::Vector3::new(0., 0., 0.);
    let focal_length = 0.030;
    let pixel_pitch = 0.000002;
    let camera_width_px = 101;
    let camera_height_px = 101;

    let camera_rays = (0..camera_height_px)
        .flat_map(|row| {
            (0..camera_width_px).map(move |col| {
                nalgebra::UnitVector3::new_normalize(nalgebra::Vector3::new(
                    pixel_pitch * (col as f32 - (camera_width_px as f32) / 2.),
                    pixel_pitch * (row as f32 - (camera_height_px as f32) / 2.),
                    focal_length,
                ))
            })
        })
        .collect::<Vec<_>>();

    let sphere = ray::Object::new(
        ray::Shape::Sphere(ray::Sphere::new(nalgebra::Vector3::new(0., 0., 1000.), 1.)),
        [255, 0, 0],
    );
    let plane = ray::Object::new(
        ray::Shape::Plane(ray::Plane::new(
            nalgebra::Vector3::new(0., 0., 1000.),
            nalgebra::UnitVector3::new_normalize(nalgebra::Vector3::new(2., 0., -1.)),
        )),
        [0, 255, 0],
    );
    let sphere2 = ray::Object::new(
        ray::Shape::Sphere(ray::Sphere::new(nalgebra::Vector3::new(1., 1.5, 900.), 1.)),
        [0, 0, 255],
    );

    let shapes = [sphere, plane, sphere2];

    let intersections = camera_rays
        .iter()
        .map(|ray| {
            let camera_ray = ray::Ray::new(camera_location, *ray);
            shapes
                .iter()
                .filter_map(|shape| {
                    shape
                        .closest_ray_intersection(&camera_ray, 0.0001)
                        .map(|dist| (dist, shape.color()))
                })
                .min_by(|a, b| a.0.total_cmp(&b.0))
        })
        .collect::<Vec<_>>();

    let (max, min) =
        intersections.iter().fold(
            (f32::MIN, f32::MAX),
            |(max, min), distance| match distance {
                Some((distance, _)) => (distance.max(max), distance.min(min)),
                None => (max, min),
            },
        );

    println!("{} -> {}", min, max);

    let mut distance_map = image::RgbImage::new(camera_width_px, camera_height_px);
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
        .save("distance.tiff")
        .expect("able to save distance_map");

    let mut color_map = image::RgbImage::new(camera_width_px, camera_height_px);
    color_map.pixels_mut().enumerate().for_each(|(i, rgb)| {
        if let Some((_, color)) = intersections[i] {
            rgb[0] = color[0];
            rgb[1] = color[1];
            rgb[2] = color[2];
        }
    });

    color_map
        .save("color.tiff")
        .expect("able to save color_map");
}
