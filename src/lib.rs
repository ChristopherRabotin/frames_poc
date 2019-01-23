extern crate nalgebra;

use nalgebra::UnitQuaternion;
use std::collections::HashMap;

pub trait Body {
    type FrameType;

    fn frames(&self) -> Vec<&Box<Self::FrameType>>;
}

pub struct CelestialBody {
    gm: f64,
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

#[derive(Copy, Clone, Debug)]
pub struct State<F>
where
    F: Frame,
{
    gm: f64, // Set to zero if Frame is NOT CelestialFrame
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub vx: f64,
    pub vy: f64,
    pub vz: f64,
    pub ax: f64,
    pub ay: f64,
    pub az: f64,
    /// The frame will later allow for coordinate frame transformations.
    pub frame: F,
}

impl<F> State<F>
where
    F: Frame,
{
    pub fn from_position_velocity<G>(
        x: f64,
        y: f64,
        z: f64,
        vx: f64,
        vy: f64,
        vz: f64,
        frame: G,
    ) -> State<G>
    where
        G: Frame,
    {
        State {
            gm: frame.gm(),
            x,
            y,
            z,
            vx,
            vy,
            vz,
            ax: 0.0,
            ay: 0.0,
            az: 0.0,
            frame,
        }
    }
}

impl State<CelestialFrame> {
    /// Returns the magnitude of the radius vector in km
    pub fn rmag(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    /// Returns the magnitude of the velocity vector in km/s
    pub fn vmag(&self) -> f64 {
        (self.vx.powi(2) + self.vy.powi(2) + self.vz.powi(2)).sqrt()
    }
    pub fn energy(&self) -> f64 {
        self.vmag().powi(2) / 2.0 - self.gm / self.rmag()
    }
}

impl State<SpaceraftFrame> {
    pub fn from_position<G>(x: f64, y: f64, z: f64, frame: G) -> State<G>
    where
        G: Frame,
    {
        State {
            gm: frame.gm(),
            x,
            y,
            z,
            vx: 0.0,
            vy: 0.0,
            vz: 0.0,
            ax: 0.0,
            ay: 0.0,
            az: 0.0,
            frame,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
