// Defines Cosm, from the Greek word for "world" or "universe".

extern crate petgraph;

use crate::exb::interpolation::StateData::{EqualStates, VarwindowStates};
use crate::exb::Ephemeris;
use crate::frames::*;
use crate::fxb::Frame;
use crate::state::State;
use crate::{load_ephemeris, load_frames};
use nalgebra::UnitQuaternion;
// use petgraph::Graph;
use std::collections::HashMap;

pub struct Cosm {
    ephemerides: HashMap<(i32, String), Ephemeris>,
    frames: HashMap<(i32, String), Frame>,
    geoids: HashMap<(i32, String), Geoid>, // TODO: Change to Graph (must confirm traverse)
}

#[derive(Debug)]
pub enum CosmError {
    ObjectNotFound(i32, String),
    NoInterpolationData(i32, String),
    NoStateData(i32, String),
}

impl Cosm {
    /// Builds a Cosm from the *XB files. Path should _not_ contain file extension.
    pub fn from_xb(filename: &str) -> Cosm {
        let mut cosm = Cosm {
            ephemerides: HashMap::new(),
            frames: HashMap::new(),
            geoids: HashMap::new(),
        };

        let ephemerides = load_ephemeris(&(filename.to_string() + ".exb"));
        for ephem in ephemerides {
            let id = ephem.id.clone().unwrap();
            cosm.ephemerides.insert((id.number, id.name), ephem);
        }

        for frame in load_frames(&(filename.to_string() + ".fxb")) {
            let id = frame.id.clone().unwrap();
            cosm.frames.insert((id.number, id.name), frame.clone());

            // Build the Geoid frames -- assume all frames are geoids if they have a GM parameter
            let exb_id = frame.exb_id.clone().unwrap();
            let exb_tpl = (exb_id.number, exb_id.name.clone());
            if let Some(ephem) = cosm.ephemerides.get(&exb_tpl) {
                // Ephemeris exists
                if let Some(gm) = ephem.ephem_parameters.get("GM") {
                    // It's a geoid, and we assume everything else is there
                    let geoid = Geoid {
                        frame_id: exb_id.clone(),
                        gm: gm.value,
                        flattening: ephem.ephem_parameters.get("Flattening").unwrap().value,
                        equatorial_radius: ephem
                            .ephem_parameters
                            .get("Equatorial radius")
                            .unwrap()
                            .value,
                        semi_major_radius: if exb_id.name == "EARTH BARYCENTER" {
                            println!("FOUND");
                            6378.1370
                        } else {
                            ephem
                                .ephem_parameters
                                .get("Equatorial radius")
                                .unwrap()
                                .value
                        },
                    };

                    cosm.geoids.insert(exb_tpl, geoid);
                }
            } else if exb_id.number == 0 {
                // Solar System Barycenter
                cosm.geoids
                    .insert(exb_tpl, Geoid::perfect_sphere(exb_id, 1.327_124_400_18e20));
            }
        }

        cosm
    }

    pub fn state<B: Body>(&self, exb: EXBID, jde: f64, frame: B) -> Result<State<B>, CosmError> {
        let ephem =
            self.ephemerides
                .get(&(exb.number, exb.name))
                .ok_or(CosmError::ObjectNotFound(
                    exb.number,
                    "exb.name".to_string(),
                ))?;

        // Compute the position
        // TODO: Maybe should this cache the previous ephemeris retrieved?
        let interp = ephem
            .clone()
            .interpolator
            .ok_or(CosmError::NoInterpolationData(
                exb.number,
                "exb.name".to_string(),
            ))?;

        let start_mod_julian: f64 = interp.start_mod_julian;
        let coefficient_count: usize = interp.position_degree as usize;

        let exb_states = match interp
            .state_data
            .ok_or(CosmError::NoStateData(exb.number, "exb.name".to_string()))?
        {
            EqualStates(states) => states.clone(),
            VarwindowStates(_) => panic!("var window not yet supported by Cosm"),
        };

        let interval_length: f64 = exb_states.window_duration;

        let delta_jde = jde - start_mod_julian;
        let index_f = (delta_jde / interval_length).round();
        let offset = delta_jde - index_f * interval_length;
        let index = index_f as usize;

        let pos_coeffs = &exb_states.position[index];

        let mut interp_t = vec![0.0; coefficient_count];
        let t1 = 2.0 * offset / interval_length - 1.0;
        interp_t[0] = 1.0;
        interp_t[1] = t1;
        for i in 2..coefficient_count {
            interp_t[i] = (2.0 * t1) * interp_t[i - 1] - interp_t[i - 2];
        }

        // XXX: This uses the positions to compute the velocity.
        let mut interp_dt = vec![0.0; coefficient_count];
        interp_dt[0] = 0.0;
        interp_dt[1] = 1.0;
        interp_dt[2] = t1 + t1;
        for i in 3..coefficient_count {
            interp_dt[i] = (2.0 * t1) * interp_dt[i - 1] - interp_dt[i - 2] + 2.0 * interp_t[i - 1];
        }

        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        let mut vx = 0.0;
        let mut vy = 0.0;
        let mut vz = 0.0;

        for (idx, pos_factor) in interp_t.iter().enumerate() {
            let vel_factor = interp_dt[idx];
            x += pos_factor * pos_coeffs.x[idx];
            y += pos_factor * pos_coeffs.y[idx];
            z += pos_factor * pos_coeffs.z[idx];
            vx += vel_factor * pos_coeffs.x[idx];
            vy += vel_factor * pos_coeffs.y[idx];
            vz += vel_factor * pos_coeffs.z[idx];
        }

        let ref_frame = ephem.ref_frame.clone().unwrap();
        println!("{:?}", ref_frame);
        // Get the Geoid associated with the ephemeris frame
        let storage_geoid = self.geoids.get(&(ref_frame.number, ref_frame.name));
        println!("{:?}", storage_geoid);

        // BUG: This does not perform any frame transformation
        Ok(State::<B>::from_position_velocity(
            x, y, z, vx, vy, vz, frame,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosm() {
        let cosm = Cosm::from_xb("./de438s");
        for key in cosm.frames.keys() {
            println!("frame -- {:?}", key);
        }
        for ek in cosm.ephemerides.keys() {
            println!(
                "ephem -- {:?} {:?}",
                ek, cosm.ephemerides[&ek].ephem_parameters
            );
        }
        for ek in cosm.geoids.keys() {
            println!("geoid -- {:?} {:?}", ek, cosm.geoids[&ek]);
        }

        let sun_id = EXBID {
            number: 10,
            name: "Sun".to_string(),
        };

        let out_body = cosm.geoids[&(0, "Solar System Barycenter".to_string())].clone();

        println!("{:?}", cosm.state(sun_id, 2474160.13175, out_body));
    }
}

/*
let mut og = Graph::new();
let ssbj2k_e = og.add_node(ssbj2k.xb_id);
let eci_e = og.add_node(eci.xb_id);
let ecef_e = og.add_node(ecef.xb_id);
og.add_edge(ssbj2k_e, eci_e, 2);
og.add_edge(eci_e, ecef_e, 1);
GraphMemCosm { graph: og }
*/
