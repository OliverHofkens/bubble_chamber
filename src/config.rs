use amethyst::core::nalgebra::Vector3;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct SimulationConfig {
    pub chamber: ChamberConfig,
    pub magnetic_field: MagneticFieldConfig,
    pub particles: MultiParticlesConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChamberConfig {
    pub width: f32,
    pub height: f32,
}

impl Default for ChamberConfig {
    fn default() -> Self {
        ChamberConfig {
            width: 100.0,
            height: 100.0,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MagneticFieldConfig {
    pub field: Vector3<f32>,
}

impl Default for MagneticFieldConfig {
    fn default() -> Self {
        MagneticFieldConfig {
            field: Vector3::new(0.0, 0.0, 2.0),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MultiParticlesConfig {
    pub at_start: Vec<ParticleConfig>,
}

impl Default for MultiParticlesConfig {
    fn default() -> Self {
        let particle = ParticleConfig::default();
        MultiParticlesConfig {
            at_start: vec![particle],
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ParticleConfig {
    pub charges: [usize; 3],
    pub location: Vector3<f32>,
    pub velocity: Vector3<f32>,
}

impl Default for ParticleConfig {
    fn default() -> Self {
        ParticleConfig {
            charges: [10, 10, 10],
            location: Vector3::new(0.0, 1080.0 / 2.0, 0.0),
            velocity: Vector3::new(500.0, 0.0, 0.0),
        }
    }
}
