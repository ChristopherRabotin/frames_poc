use crate::fxb::frame::Identifier;
use nalgebra::UnitQuaternion;

pub struct Frame<B: Body> {
    pub center: B,
    pub xb_id: Identifier,
}

pub trait Body {
    fn id(&self) -> &Identifier;
}

#[derive(Clone, Debug)]
pub struct Geoid {
    pub id: Identifier,
    pub gm: f64,
    pub flattening: f64,
    pub equatorial_radius: f64,
    pub semi_major_radius: f64,
}

impl Geoid {
    pub fn perfect_sphere(id: Identifier, gm: f64) -> Geoid {
        Geoid {
            id,
            gm,
            flattening: 0.0,
            equatorial_radius: 0.0,
            semi_major_radius: 0.0,
        }
    }
}

impl Body for Geoid {
    fn id(&self) -> &Identifier {
        &self.id
    }
}

pub struct Spacecraft {
    pub id: Identifier,
}

impl Body for Spacecraft {
    fn id(&self) -> &Identifier {
        &self.id
    }
}
