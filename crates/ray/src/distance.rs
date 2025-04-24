#[derive(Copy, Clone, Debug)]
pub struct Distance(f32);

const UM_FACTOR: f32 = 1e6;
const MM_FACTOR: f32 = 1e3;

impl Distance {
    #[inline(always)]
    pub const fn from_m(m: f32) -> Self {
        Self(m)
    }

    #[inline(always)]
    pub const fn from_um(um: f32) -> Self {
        Self(um / UM_FACTOR)
    }

    #[inline(always)]
    pub const fn from_mm(mm: f32) -> Self {
        Self(mm / MM_FACTOR)
    }

    #[inline(always)]
    pub const fn m(&self) -> f32 {
        self.0
    }

    #[inline(always)]
    pub const fn um(&self) -> f32 {
        self.0 * UM_FACTOR
    }

    #[inline(always)]
    pub const fn mm(&self) -> f32 {
        self.0 * MM_FACTOR
    }
}
