use nalgebra::UnitQuaternion;

pub trait Frame
where
    Self: Sized,
{
    fn gm(&self) -> f64;
    fn parent(self) -> Option<Box<Self>>;
    fn rotation_to_parent(&self, at: f64) -> UnitQuaternion<f64>;
}

pub struct CelestialFrame {
    parent: Option<Box<CelestialFrame>>,
    rotation: UnitQuaternion<f64>,
    body: Box<CelestialBody>,
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
    parent: Option<Box<SpaceraftFrame>>,
    rotation: UnitQuaternion<f64>,
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
    parent: Option<Box<GeoidFrame>>,
    rotation: UnitQuaternion<f64>,
    body: Box<CelestialFrame>,
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

    fn frames(&self) -> Vec<&Box<Self::FrameType>>;
}

pub struct CelestialBody {
    pub gm: f64,
    frames_map: HashMap<String, Box<CelestialFrame>>,
}

impl Body for CelestialBody {
    type FrameType = CelestialFrame;

    fn frames(&self) -> Vec<&Box<CelestialFrame>> {
        let frames: Vec<&Box<CelestialFrame>> =
            self.frames_map.iter().map(|(_, frame)| frame).collect();
        frames
    }
}
