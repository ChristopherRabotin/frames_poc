use crate::frames::*;
use crate::{between_0_360, between_pm_180};

use nalgebra::Vector3;

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

    /// Returns the magnitude of the radius vector in km
    pub fn rmag(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    /// Returns the magnitude of the velocity vector in km/s
    pub fn vmag(&self) -> f64 {
        (self.vx.powi(2) + self.vy.powi(2) + self.vz.powi(2)).sqrt()
    }
}

impl State<CelestialFrame> {
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

impl State<GeoidFrame> {
    /// Creates a new State from the geodetic latitude (φ), longitude (λ) and height with respect to Earth's ellipsoid.
    ///
    /// **Units:** degrees, degrees, km
    /// NOTE: This computation differs from the spherical coordinates because we consider the flattening of Earth.
    /// Reference: G. Xu and Y. Xu, "GPS", DOI 10.1007/978-3-662-50367-6_2, 2016
    pub fn from_geodesic(
        latitude: f64,
        longitude: f64,
        height: f64,
        frame: GeoidFrame,
    ) -> State<GeoidFrame> {
        let e2 = 2.0 * frame.flattening - frame.flattening.powi(2);
        let (sin_long, cos_long) = longitude.to_radians().sin_cos();
        let (sin_lat, cos_lat) = latitude.to_radians().sin_cos();
        // page 144
        let c_earth = frame.semi_major_radius / ((1.0 - e2 * sin_lat.powi(2)).sqrt());
        let s_earth = (frame.semi_major_radius * (1.0 - frame.flattening).powi(2))
            / ((1.0 - e2 * sin_lat.powi(2)).sqrt());
        let ri = (c_earth + height) * cos_lat * cos_long;
        let rj = (c_earth + height) * cos_lat * sin_long;
        let rk = (s_earth + height) * sin_lat;
        let radius = Vector3::new(ri, rj, rk);
        let velocity = Vector3::new(0.0, 0.0, frame.rotation_rate).cross(&radius);
        State::<GeoidFrame>::from_position_velocity(
            radius[(0, 0)],
            radius[(1, 0)],
            radius[(2, 0)],
            velocity[(0, 0)],
            velocity[(1, 0)],
            velocity[(2, 0)],
            frame,
        )
    }

    /// Creates a new ECEF state at the provided position.
    ///
    /// NOTE: This has the same container as the normal State. Hence, we set the velocity at zero.
    pub fn from_position(i: f64, j: f64, k: f64, frame: GeoidFrame) -> State<GeoidFrame> {
        State::<GeoidFrame>::from_position_velocity(i, j, k, 0.0, 0.0, 0.0, frame)
    }

    /// Returns the I component of this ECEF frame
    pub fn ri(&self) -> f64 {
        self.x
    }

    /// Returns the J component of this ECEF frame
    pub fn rj(&self) -> f64 {
        self.y
    }

    /// Returns the K component of this ECEF frame
    pub fn rk(&self) -> f64 {
        self.z
    }

    /// Returns the geodetic longitude (λ) in degrees. Value is between 0 and 360 degrees.
    ///
    /// Although the reference is not Vallado, the math from Vallado proves to be equivalent.
    /// Reference: G. Xu and Y. Xu, "GPS", DOI 10.1007/978-3-662-50367-6_2, 2016
    pub fn geodetic_longitude(&self) -> f64 {
        between_0_360(self.y.atan2(self.x).to_degrees())
    }

    /// Returns the geodetic latitude (φ) in degrees. Value is between -180 and +180 degrees.
    ///
    /// Reference: Vallado, 4th Ed., Algorithm 12 page 172.
    pub fn geodetic_latitude(&self) -> f64 {
        let eps = 1e-12;
        let max_attempts = 20;
        let mut attempt_no = 0;
        let r_delta = (self.x.powi(2) + self.y.powi(2)).sqrt();
        let mut latitude = (self.z / self.rmag()).asin();
        let e2 = self.frame.flattening * (2.0 - self.frame.flattening);
        loop {
            attempt_no += 1;
            let c_earth =
                self.frame.semi_major_radius / ((1.0 - e2 * (latitude).sin().powi(2)).sqrt());
            let new_latitude = (self.z + c_earth * e2 * (latitude).sin()).atan2(r_delta);
            if (latitude - new_latitude).abs() < eps {
                return between_pm_180(new_latitude.to_degrees());
            } else if attempt_no >= max_attempts {
                println!(
                    "geodetic latitude failed to converge -- error = {}",
                    (latitude - new_latitude).abs()
                );
                return between_pm_180(new_latitude.to_degrees());
            }
            latitude = new_latitude;
        }
    }

    /// Returns the geodetic height in km.
    ///
    /// Reference: Vallado, 4th Ed., Algorithm 12 page 172.
    pub fn geodetic_height(&self) -> f64 {
        let e2 = self.frame.flattening * (2.0 - self.frame.flattening);
        let latitude = self.geodetic_latitude().to_radians();
        let sin_lat = latitude.sin();
        if (latitude - 1.0).abs() < 0.1 {
            // We are near poles, let's use another formulation.
            let s_earth = (self.frame.semi_major_radius * (1.0 - self.frame.flattening).powi(2))
                / ((1.0 - e2 * sin_lat.powi(2)).sqrt());
            self.z / latitude.sin() - s_earth
        } else {
            let c_earth = self.frame.semi_major_radius / ((1.0 - e2 * sin_lat.powi(2)).sqrt());
            let r_delta = (self.x.powi(2) + self.y.powi(2)).sqrt();
            r_delta / latitude.cos() - c_earth
        }
    }
}
