use amethyst::core::nalgebra::Vector3;

/// The global magnetic field
pub struct MagneticField {
    pub field: Vector3<f32>,
}

pub struct SVGBuilder {
    pub paths: Vec<Vec<[f32; 2]>>,
}

impl Default for SVGBuilder {
    fn default() -> Self {
        SVGBuilder { paths: Vec::new() }
    }
}
