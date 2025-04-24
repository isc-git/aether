/// angle abstracted from units,
/// internally represented as a radian
pub struct Angle(f32);

impl Angle {
    pub const ZERO: Self = Self(0.);

    #[inline(always)]
    pub fn from_rads(rads: f32) -> Self {
        Self(rads)
    }

    #[inline(always)]
    pub fn from_degs(degs: f32) -> Self {
        Self(degs.to_radians())
    }

    #[inline(always)]
    pub fn degs(&self) -> f32 {
        self.0.to_degrees()
    }

    #[inline(always)]
    pub fn rads(&self) -> f32 {
        self.0
    }

    #[inline(always)]
    pub fn cos(&self) -> f32 {
        self.0.cos()
    }

    #[inline(always)]
    pub fn sin(&self) -> f32 {
        self.0.sin()
    }

    #[inline(always)]
    pub fn tan(&self) -> f32 {
        self.0.tan()
    }
}

impl std::ops::Mul<f32> for Angle {
    type Output = Angle;

    fn mul(self, rhs: f32) -> Self::Output {
        Angle(self.0 * rhs)
    }
}

impl std::fmt::Display for Angle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.03}°", self.degs())
    }
}
