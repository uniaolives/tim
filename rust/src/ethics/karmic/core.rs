use serde::{Serialize, Deserialize};
use crate::geometric_interrogation::Vector;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Grade(u8);

impl From<u8> for Grade {
    fn from(g: u8) -> Self {
        Grade(g.clamp(1, 99))
    }
}

impl Grade {
    pub fn value(&self) -> u8 {
        self.0
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SoulState {
    pub grade: Grade,
    pub service_ratio: f64, // STO:STS ratio (0.0-1.0)
    pub wisdom: u8,         // 0-100
    pub resurrection_impact: i64,
}

impl SoulState {
    pub fn new() -> Self {
        SoulState {
            grade: Grade::from(1),
            service_ratio: 0.5,
            wisdom: 50,
            resurrection_impact: 0,
        }
    }
}

pub struct KarmicManifold {
    pub dimensions: usize,
    pub curvature: f64,
    pub phi: f64,
}

impl KarmicManifold {
    pub fn new(dimensions: usize, curvature: f64, phi: f64) -> Self {
        KarmicManifold {
            dimensions,
            curvature,
            phi,
        }
    }

    pub fn to_vector_1024d(&self, grade: &Grade) -> Vector<1024> {
        let mut components = [0.0; 1024];
        let grade_val = grade.value() as f64 / 100.0;

        // Isomorfismo: mapear grade para subespaços baseados na densidade ontológica
        if grade.value() < 25 {
            for i in 0..256 { components[i] = grade_val; }
        } else if grade.value() < 50 {
            for i in 0..512 { components[i] = grade_val; }
        } else if grade.value() < 75 {
            for i in 0..768 { components[i] = grade_val; }
        } else {
            for i in 0..1024 { components[i] = grade_val; }
        }

        Vector::new(components)
    }
}
