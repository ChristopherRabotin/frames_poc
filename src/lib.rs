extern crate nalgebra;
use nalgebra::geometry::UnitQuaternion;
use nalgebra::Vector3;

use std::fmt::Debug;
pub trait Frame: Debug + Sized {
    fn parent() -> Option<Self>;
    fn name() -> &'static str;
    fn rotation_to_parent<M: FrameMgr>(at_time: f64, mgr: &M) -> UnitQuaternion<f64>;
    fn translation_to_parent<M: FrameMgr>(at_time: f64, mgr: &M) -> Vector3<f64>;
    // fn parent<F: Frame>() -> Option<F>;
}

pub trait CelestialFrame: Frame {
    type Body: CelestialBody;
}

pub trait FrameMgr {
    fn frame_from_name<F: Frame>(name: &'static str) -> F;

    fn celestial_frame<C: CelestialFrame>(name: &'static str) -> C;
    // fn convert_state<F: Frame, T: Frame>(from: State<F>) -> State<T>;
}

pub struct SimpleFrameMgr;
impl FrameMgr for SimpleFrameMgr {
    fn frame_from_name<F: Frame>(name: &'static str) -> F {
        if name == "RIC" {
            return RIC {};
        }
        panic!("unknown frame");
    }

    fn celestial_frame<C: CelestialFrame>(name: &'static str) -> C {
        if name == "ECI" {
            return ECI {};
        } else if name == "SSB" {
            return SSB {};
        }
        panic!("unknown frame");
    }
}

/// `CelestialBody` represents a celestial body.
///
/// Note that all planets are defined as types. This leverages higher speed of execution via monomorphism.
/// The `CelestialBody`s provided in nyx use the same values as those in [GMAT 2016a](https://github.com/ChristopherRabotin/GMAT/blob/37201a6290e7f7b941bc98ee973a527a5857104b/src/base/util/GmatDefaults.hpp).
/// NOTE: There is no Pluto defined in nyx because it isn't a planet: it's a collection of three (four?) small rocks orbiting each other.
pub trait CelestialBody {
    /// Returns the gravitional parameter of the given body. **Unit**: km<sup>3</sup>/s<sup>2</sup>
    fn gm() -> f64;
    /// Returns the equatorial radius of this celestial object.
    fn eq_radius() -> f64;
    /// Returns the flattening of this celestial object.
    fn flattening() -> f64;
}

/// Planet Earth as defined in [GMAT 2016a](https://github.com/ChristopherRabotin/GMAT/blob/37201a6290e7f7b941bc98ee973a527a5857104b/src/base/util/GmatDefaults.hpp).
pub struct EARTH;

impl EARTH {
    /// Defines the semi major radius of the ellipsoid of Earth, as per WGS84, in km.
    pub fn semi_major_radius() -> f64 {
        6378.1370
    }

    /// The rotation rate of Earth, in radians per seconds; [source](http://hpiers.obspm.fr/eop-pc/models/constants.html).
    pub fn rotation_rate() -> f64 {
        7.292_115_146_706_4e-5
    }
}

impl CelestialBody for EARTH {
    fn gm() -> f64 {
        398_600.441_5
    }
    fn eq_radius() -> f64 {
        6_378.136_3
    }
    fn flattening() -> f64 {
        // From [EMG2008](http://earth-info.nga.mil/GandG/wgs84/gravitymod/egm2008/egm08_wgs84.html)
        0.003_352_810_664_747_480_5
    }
}

#[derive(Copy, Clone, Debug)]
pub struct State<F>
where
    F: CelestialFrame,
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

impl<F> State<F>
where
    F: CelestialFrame,
{
    /// Creates a new State around the provided CelestialBody
    ///
    /// **Units:** km, km, km, km/s, km/s, km/s
    pub fn from_cartesian(x: f64, y: f64, z: f64, vx: f64, vy: f64, vz: f64, frame: F) -> State<F>
    where
        F: CelestialFrame,
    {
        State {
            gm: F::Body::gm(),
            x,
            y,
            z,
            vx,
            vy,
            vz,
            frame,
        }
    }
    /// Converts this state to a state in the destination frame (`other`).
    ///
    /// Reference: Vallado, 4th Ed., page 167, equation 3-27.
    /// Note that we compute the derivative of the DCM by taking the difference between
    /// said DCMs at a 0.1 second interval.
    pub fn in_frame<G: CelestialFrame, M: FrameMgr>(self, other: G, mgr: M) -> State<G> {
        State {
            gm: self.gm,
            x: 0.0,
            y: 0.0,
            z: 0.0,
            vx: 0.0,
            vy: 0.0,
            vz: 0.0,
            frame: other,
        }
    }
}

#[derive(Debug)]
pub struct RIC;
impl Frame for RIC {
    fn parent() -> Option<Self> {
        None
    }

    fn name() -> &'static str {
        "RIC"
    }

    fn rotation_to_parent<M: FrameMgr>(_at_time: f64, _mgr: &M) -> UnitQuaternion<f64> {
        UnitQuaternion::new(Vector3::new(0.0, 0.0, 0.0))
    }

    fn translation_to_parent<M: FrameMgr>(at_time: f64, mgr: &M) -> Vector3<f64> {
        Vector3::new(0.0, 0.0, 0.0)
    }
}

#[derive(Debug)]
pub struct ECI;
impl Frame for ECI {
    fn parent() -> Option<Self> {
        SSB {}
    }

    fn name() -> &'static str {
        "ECI"
    }

    fn rotation_to_parent<M: FrameMgr>(_at_time: f64, _mgr: &M) -> UnitQuaternion<f64> {
        UnitQuaternion::new(Vector3::new(0.0, 0.0, 0.0))
    }

    fn translation_to_parent<M: FrameMgr>(at_time: f64, mgr: &M) -> Vector3<f64> {
        Vector3::new(0.0, 0.0, 0.0)
    }
}
impl CelestialFrame for ECI {
    type Body = EARTH;
}

#[derive(Debug)]
pub struct SSB;

impl Frame for SSB {
    fn parent() -> Option<Self> {
        None
    }

    fn name() -> &'static str {
        "SSB"
    }

    fn rotation_to_parent<M: FrameMgr>(_at_time: f64, _mgr: &M) -> UnitQuaternion<f64> {
        UnitQuaternion::new(Vector3::new(0.0, 0.0, 0.0))
    }

    fn translation_to_parent<M: FrameMgr>(at_time: f64, mgr: &M) -> Vector3<f64> {
        Vector3::new(0.0, 0.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use crate::{State, ECI};
        let cart = State::from_cartesian(
            -2436.45,
            -2436.45,
            6891.037,
            5.088611,
            -5.088611,
            0.0,
            ECI {},
        );
        println!("{:?}", cart);

        let cart = State::<ECI> {
            x: -2436.45,
            y: -2436.45,
            z: 6891.037,
            vx: 5.088611,
            vy: -5.088611,
            vz: 0.0,
        };
    }
}
