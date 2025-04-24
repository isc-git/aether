fn main() {
    let camera = ray::camera::Camera::new(
        101,
        101,
        ray::distance::Distance::from_um(2.0),
        ray::distance::Distance::from_m(0.03),
    );
    let camera_location = nalgebra::Vector3::new(0., 0., 0.);

    let camera_rays = (0..camera.width_px() * camera.height_px())
        .map(|i| {
            let row = i / camera.width_px();
            let col = i % camera.width_px();
            nalgebra::UnitVector3::new_normalize(
                camera.pixel_to_camera_vector(col as f32, row as f32),
            )
        })
        .collect::<Vec<_>>();

    let sphere = ray::object::Object::new(
        ray::shapes::Shape::Sphere(ray::shapes::sphere::Sphere::new(
            nalgebra::Vector3::new(0., 0., 1000.),
            1.,
        )),
        [255, 0, 0],
    );
    let plane = ray::object::Object::new(
        ray::shapes::Shape::Plane(ray::shapes::plane::Plane::new(
            nalgebra::Vector3::new(0., 0., 1000.),
            nalgebra::UnitVector3::new_normalize(nalgebra::Vector3::new(2., 0., -1.)),
        )),
        [0, 255, 0],
    );
    let sphere2 = ray::object::Object::new(
        ray::shapes::Shape::Sphere(ray::shapes::sphere::Sphere::new(
            nalgebra::Vector3::new(1., 1.5, 900.),
            1.,
        )),
        [0, 0, 255],
    );

    let shapes = [sphere, plane, sphere2];

    let intersections = camera_rays
        .iter()
        .map(|ray| {
            let camera_ray = ray::ray::Ray::new(camera_location, *ray);
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
        .save("distance.tiff")
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
        .save("color.tiff")
        .expect("able to save color_map");
}
