use crate::angle;

pub struct Camera {
    width_px: f32,
    height_px: f32,
    pixel_pitch: crate::distance::Distance,
    focal_length: crate::distance::Distance,
}

impl Camera {
    pub fn new(
        width_px: u32,
        height_px: u32,
        pixel_pitch: crate::distance::Distance,
        focal_length: crate::distance::Distance,
    ) -> Self {
        debug_assert!(width_px > 0, "width of a camera cannot be <= 0");
        debug_assert!(width_px > 0, "width of a camera cannot be <= 0");
        debug_assert!(
            pixel_pitch.m() > 0.,
            "pixel pitch of a camera cannot be <= 0.0"
        );

        Self {
            width_px: width_px as f32,
            height_px: height_px as f32,
            pixel_pitch,
            focal_length,
        }
    }

    /// Determines the ray from the camera from the pixel location.
    /// Note that values outside of the defined camera array valid.
    ///
    /// x increases right
    /// y increases down
    /// z increases away from camera
    pub fn pixel_to_camera_vector(&self, x: f32, y: f32) -> nalgebra::Vector3<f32> {
        let x = self.pixel_pitch.m() * (x - self.width_px / 2.);
        let y = self.pixel_pitch.m() * (y - self.height_px / 2.);
        let z = self.focal_length.m();

        nalgebra::Vector3::new(x, y, z)
    }

    pub fn width_px(&self) -> u32 {
        self.width_px as u32
    }

    pub fn height_px(&self) -> u32 {
        self.height_px as u32
    }

    /// horizontal field of view of the camera
    pub fn hfov(&self) -> angle::Angle {
        let half_width = 0.5 * self.pixel_pitch.m() * self.width_px;
        let rads = 2. * (half_width / self.focal_length.m()).atan();
        angle::Angle::from_rads(rads)
    }

    /// vertical field of view of the camera
    pub fn vfov(&self) -> angle::Angle {
        let half_height = 0.5 * self.pixel_pitch.m() * self.height_px;
        let rads = 2. * (half_height / self.focal_length.m()).atan();
        angle::Angle::from_rads(rads)
    }
}
