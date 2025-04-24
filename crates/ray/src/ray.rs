pub struct Ray {
    origin: nalgebra::Vector3<f32>,
    // assume normalized
    direction: nalgebra::UnitVector3<f32>,
}

impl Ray {
    pub fn new(origin: nalgebra::Vector3<f32>, direction: nalgebra::UnitVector3<f32>) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> &nalgebra::Vector3<f32> {
        &self.origin
    }

    pub fn direction(&self) -> &nalgebra::Vector3<f32> {
        &self.direction
    }
}
