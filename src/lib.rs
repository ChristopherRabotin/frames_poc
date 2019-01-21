extern crate nalgebra;

use nalgebra::UnitQuaternion;
use std::collections::HashMap;

pub trait Body {
    fn frames<F: Frame>() -> [F];
}

pub struct CelestialBody<'a, 'b> {
    gm: f64,
    eq_radius: f64,
    flattening: f64,
    frames_map: HashMap<String, CelestialFrame<'a, 'b>>,
}

pub trait Frame
where
    Self: Sized,
{
    fn parent(&self) -> Option<Self>;
    fn rotation_to_parent(&self, at: f64) -> UnitQuaternion<f64>;
}

pub struct CelestialFrame<'a, 'b> {
    parent: Option<CelestialFrame<'a, 'a>>,
    rotation: UnitQuaternion<f64>,
    body: &'b CelestialBody<'a, 'b>,
}

impl<'a, 'b> Frame for CelestialFrame<'a, 'b> {
    fn parent(&self) -> Option<CelestialFrame<'a, 'b>> {
        self.parent
    }
    fn rotation_to_parent(&self, at: f64) -> UnitQuaternion<f64> {
        self.rotation
    }
}

pub struct SpaceraftFrame;

#[derive(Copy, Clone, Debug)]
pub struct State<F>
where
    F: Frame,
{
    gm: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub vx: f64,
    pub vy: f64,
    pub vz: f64,
    /// The frame will later allow for coordinate frame transformations.
    pub frame: F,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
