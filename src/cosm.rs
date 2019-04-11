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
    ObjectNotFound,
    NoInterpolationData,
    NoStateData,
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

    pub fn position<B: Body>(&self, exb: EXBID, jde: f64, frame: B) -> Result<State<B>, CosmError> {
        let ephem = self
            .ephemerides
            .get(&(exb.number, exb.name))
            .ok_or(CosmError::ObjectNotFound)?;

        // Compute the position
        // TODO: Maybe should this cache the previous ephemeris retrieved?
        let interp = ephem
            .clone()
            .interpolator
            .ok_or(CosmError::NoInterpolationData)?;

        let start_mod_julian: f64 = interp.start_mod_julian;
        let coefficient_count: usize = interp.position_degree as usize;

        let exb_states = match interp.state_data.ok_or(CosmError::NoStateData)? {
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

        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;

        for (idx, factor) in interp_t.iter().enumerate() {
            x += factor * pos_coeffs.x[idx];
            y += factor * pos_coeffs.y[idx];
            z += factor * pos_coeffs.z[idx];
        }

        // BUG: This does not perform any frame transformation
        Ok(State::<B>::from_position(x, y, z, frame))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosm() {
        let cosm = Cosm::from_xb("./de438s");
        for key in cosm.frames.keys() {
            println!("{:?}", key);
        }
        for ephem in cosm.ephemerides.values() {
            println!("{:?}", ephem.ephem_parameters);
        }
        for geoid in cosm.geoids.values() {
            println!("{:?}", geoid);
        }

        let obj_id = EXBID {
            number: 100000,
            name: "J2000 SSB".to_string(),
        };

        let out_body = cosm.geoids[&(0, "Solar System Barycenter".to_string())].clone();

        println!("{:?}", cosm.position(obj_id, 2474160.13175, out_body));
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
