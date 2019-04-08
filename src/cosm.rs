// Defines Cosm, from the Greek word for "world" or "universe".

extern crate petgraph;

// use crate::exb::ephemeris::Identifier;
use crate::exb::Ephemeris;
use crate::frames::*;
use crate::fxb::Frame;
use crate::{load_ephemeris, load_frames};
use nalgebra::UnitQuaternion;
// use petgraph::Graph;
use std::collections::HashMap;

pub struct Cosm {
    ephemerides: HashMap<(i32, String), Ephemeris>,
    frames: HashMap<(i32, String), Frame>,
    geoids: HashMap<(i32, String), Geoid>, // TODO: Change to Graph (must confirm traverse)
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
                        id: exb_id.clone(),
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
            } else if exb_id.number == 100000 {
                // Solar System Barycenter
                cosm.geoids
                    .insert(exb_tpl, Geoid::perfect_sphere(exb_id, 1.327_124_400_18e20));
            }
        }

        cosm
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load() {
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
