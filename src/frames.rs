use nalgebra::UnitQuaternion;

pub trait Frame
where
    Self: Sized,
{
    fn gm(&self) -> f64;
    fn name(self) -> String;
}

pub struct CelestialFrame {
    pub parent: Option<Box<CelestialFrame>>,
    pub rotation: UnitQuaternion<f64>,
    pub body: &CelestialBody,
}

impl Frame for CelestialFrame {
    fn gm(&self) -> f64 {
        self.body.gm
    }
    fn parent(self) -> Option<Box<CelestialFrame>> {
        self.parent
    }
    fn rotation_to_parent(&self, at: f64) -> UnitQuaternion<f64> {
        self.rotation
    }
}

pub struct SpaceraftFrame {
    pub parent: Option<Box<SpaceraftFrame>>,
    pub rotation: UnitQuaternion<f64>,
}

impl Frame for SpaceraftFrame {
    fn gm(&self) -> f64 {
        0.0
    }
    fn parent(self) -> Option<Box<SpaceraftFrame>> {
        self.parent
    }
    fn rotation_to_parent(&self, at: f64) -> UnitQuaternion<f64> {
        self.rotation
    }
}

pub struct GeoidFrame {
    pub parent: Option<Box<GeoidFrame>>,
    pub rotation: UnitQuaternion<f64>,
    pub body: &CelestialFrame,
    pub flattening: f64,
    pub semi_major_radius: f64,
    pub rotation_rate: f64,
}

impl Frame for GeoidFrame {
    fn gm(&self) -> f64 {
        self.body.gm()
    }
    fn parent(self) -> Option<Box<GeoidFrame>> {
        self.parent
    }
    fn rotation_to_parent(&self, at: f64) -> UnitQuaternion<f64> {
        self.rotation
    }
}

// Here goes the bodies (avoids a circular dependency)
use std::collections::HashMap;

pub trait Body {
    type FrameType;

    fn name(self) -> String;
}

pub struct CelestialBody {
    pub gm: f64,
    pub name: String,
}

impl Body for CelestialBody {
    type FrameType = CelestialFrame;

    fn name(self) -> String {
        self.name
    }
}
