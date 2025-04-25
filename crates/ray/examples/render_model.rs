use std::{f32::consts::PI, fs, path};

use ray::{distance, shapes::Traceable as _};
use rayon::prelude::*;

const CAMERA_WIDTH: u32 = 1920;
const CAMERA_HEIGHT: u32 = 1080;
const CAMERA_PIXEL_PITCH: distance::Distance = distance::Distance::from_um(2.);
const CAMERA_FOCAL_LENGTH: distance::Distance = distance::Distance::from_mm(30.);

const CAMERA_LOCATION: nalgebra::Vector3<f32> = nalgebra::Vector3::new(-100., 0., 0.);

#[derive(argh::FromArgs)]
/// opens and renders a .glb
struct Args {
    #[argh(short = 'i', option)]
    /// input model path
    input: path::PathBuf,
}

fn main() {
    let args: Args = argh::from_env();

    if !args.input.exists() {
        eprintln!("input file does not exist: \"{}\"", args.input.display());
        std::process::exit(1);
    }

    if args
        .input
        .extension()
        .is_none_or(|ext| ext.to_str().is_none_or(|ext| ext != "glb"))
    {
        eprintln!(
            "input file does not have expected extention (glb): \"{}\"",
            args.input.display()
        );
        std::process::exit(1);
    }

    let camera = ray::camera::Camera::new(
        CAMERA_WIDTH,
        CAMERA_HEIGHT,
        CAMERA_PIXEL_PITCH,
        CAMERA_FOCAL_LENGTH,
    );
    let body_to_ned = nalgebra::Rotation3::from_euler_angles(0., 0., 0.);
    let camera_to_body =
        nalgebra::Rotation3::from_euler_angles(90f32.to_radians(), 0., 90f32.to_radians());
    let camera_to_ned = body_to_ned * camera_to_body;

    let camera_rays = (0..camera.width_px() * camera.height_px())
        .map(|i| {
            let row = i / camera.width_px();
            let col = i % camera.width_px();
            nalgebra::UnitVector3::new_normalize(
                camera_to_ned * camera.pixel_to_camera_vector(col as f32, row as f32),
            )
        })
        .collect::<Vec<_>>();

    let (glb_document, glb_buffers, _glb_images) =
        gltf::import(&args.input).expect("expected valid glb file");

    let mut meshes = Vec::new();
    let mut glb_positions = Vec::new();
    let mut glb_indices = Vec::new();
    for glb_mesh in glb_document.meshes() {
        for primative in glb_mesh.primitives() {
            glb_positions.clear();
            glb_indices.clear();
            let reader = primative.reader(|buffer| Some(&glb_buffers[buffer.index()]));
            if let Some(reader) = reader.read_positions() {
                glb_positions.extend(reader);
            }
            if let Some(indices) = reader.read_indices().map(|i| i.into_u32()) {
                glb_indices.extend(indices);
            }
            let mut triangles = Vec::new();
            for triangle in glb_indices.chunks(3) {
                if triangle.len() == 3 {
                    let a = glb_positions[triangle[0] as usize];
                    let b = glb_positions[triangle[1] as usize];
                    let c = glb_positions[triangle[2] as usize];
                    triangles.push(ray::shapes::triangle::Triangle::new(
                        nalgebra::Vector3::from(a),
                        nalgebra::Vector3::from(b),
                        nalgebra::Vector3::from(c),
                    ));
                } else {
                    eprintln!("got incomplete triangle");
                }
            }
            meshes.push(triangles);
        }
    }

    // creating our giff encoders
    let mut depth_giff = fs::File::create("depth.gif").expect("able to create depth.gif");
    let mut intersection_giff =
        fs::File::create("intersection.gif").expect("able to create intersection.gif");

    let mut depth_encoder = gif::Encoder::new(
        &mut depth_giff,
        CAMERA_WIDTH as u16,
        CAMERA_HEIGHT as u16,
        &[],
    )
    .expect("valid encoder");
    depth_encoder
        .set_repeat(gif::Repeat::Infinite)
        .expect("repeat is valid for a giff");

    let mut intersection_encoder = gif::Encoder::new(
        &mut intersection_giff,
        CAMERA_WIDTH as u16,
        CAMERA_HEIGHT as u16,
        &[],
    )
    .expect("valid encoder");
    intersection_encoder
        .set_repeat(gif::Repeat::Infinite)
        .expect("repeat is valid for a giff");

    let loop_time_s = 2.;
    let loop_angle = 2. * PI;
    let frame_delay_ms = 20.;
    let frame_count = (loop_time_s / (frame_delay_ms / 1000.)) as usize;
    println!("{}", frame_count);

    let angle_deltas = loop_angle / frame_count as f32;
    let angles = (0..frame_count).map(|a| a as f32 * angle_deltas);

    let axis = nalgebra::UnitVector3::new_normalize(nalgebra::Vector3::new(0., 0., 1.));

    let mut intersection_map = image::RgbImage::new(camera.width_px(), camera.height_px());
    let mut distance_map = image::RgbImage::new(camera.width_px(), camera.height_px());
    for angle in angles {
        let start = std::time::Instant::now();
        let rotation = nalgebra::UnitQuaternion::from_axis_angle(&axis, angle);

        let intersections = camera_rays
            .par_iter()
            .map(|ray| {
                let camera_ray = ray::ray::Ray::new(CAMERA_LOCATION, *ray);
                meshes
                    .iter()
                    .filter_map(|mesh| {
                        mesh.iter()
                            .filter_map(|triangle| {
                                triangle.rotate(&rotation).trace(&camera_ray, 0.0001)
                            })
                            .min_by(|a, b| a.total_cmp(b))
                    })
                    .min_by(|a, b| a.total_cmp(b))
            })
            .collect::<Vec<_>>();

        println!(
            "intersections: {}",
            (std::time::Instant::now() - start).as_secs_f32()
        );

        let (max, min) =
            intersections.iter().fold(
                (f32::MIN, f32::MAX),
                |(max, min), distance| match distance {
                    Some(distance) => (distance.max(max), distance.min(min)),
                    None => (max, min),
                },
            );
        println!("{} -> {}", min, max);
        println!(
            "max,min: {}",
            (std::time::Instant::now() - start).as_secs_f32()
        );

        intersection_map
            .pixels_mut()
            .enumerate()
            .for_each(|(i, rgb)| {
                if intersections[i].is_some() {
                    *rgb = image::Rgb([255, 255, 255]);
                } else {
                    *rgb = image::Rgb([0, 0, 0]);
                }
            });
        println!(
            "mapped intersection: {}",
            (std::time::Instant::now() - start).as_secs_f32()
        );

        distance_map
            .pixels_mut()
            .enumerate()
            .for_each(|(i, rgb)| match intersections[i] {
                Some(dist) => {
                    let value = ((1.0 - ((dist - min) / (max - min))).powf(0.6) * 255. * 3.) as u16;
                    let r = value.min(255);
                    let value = value.saturating_sub(255);
                    let g = value.min(255);
                    let value = value.saturating_sub(255);
                    let b = value;

                    rgb[0] = r as u8;
                    rgb[1] = g as u8;
                    rgb[2] = b as u8;
                }
                None => {
                    *rgb = image::Rgb([0, 0, 0]);
                }
            });
        println!(
            "mapped depth: {}",
            (std::time::Instant::now() - start).as_secs_f32()
        );

        //// for each frame
        let mut depth_frame = gif::Frame::from_rgb(
            CAMERA_WIDTH as u16,
            CAMERA_HEIGHT as u16,
            distance_map.as_raw(),
        );
        depth_frame.delay = frame_delay_ms as u16; // 10 = 100ms per frame
        depth_encoder
            .write_frame(&depth_frame)
            .expect("able to write depth frame");

        println!(
            "encoded depth: {}",
            (std::time::Instant::now() - start).as_secs_f32()
        );

        let mut intersection_frame = gif::Frame::from_rgb(
            CAMERA_WIDTH as u16,
            CAMERA_HEIGHT as u16,
            intersection_map.as_raw(),
        );
        intersection_frame.delay = frame_delay_ms as u16; // 10 = 100ms per frame
        intersection_encoder
            .write_frame(&intersection_frame)
            .expect("able to write intersection frame");

        println!(
            "encoded intersection: {}",
            (std::time::Instant::now() - start).as_secs_f32()
        );
    }
}
