// Defines Cosm, from the Greek word for "world" or "universe".

extern crate petgraph;

// use crate::exb::ephemeris::Identifier;
use crate::exb::Ephemeris;
use crate::frames::*;
use crate::fxb::Frame;
use crate::{load_ephemeris, load_frames};
use nalgebra::UnitQuaternion;
use petgraph::Graph;
use std::collections::HashMap;

pub struct Cosm {
    ephem_map: HashMap<(i32, String), Ephemeris>,
    frame_map: HashMap<(i32, String), Frame>,
    geoid_graph: Graph<(i32, String), Geoid>,
}

impl Cosm {
    /// Builds a Cosm from the *XB files. Path should _not_ contain file extension.
    pub fn from_xb(filename: &str) -> Cosm {
        let mut cosm = Cosm {
            ephem_map: HashMap::new(),
            frame_map: HashMap::new(),
            geoid_graph: Graph::new(),
        };

        let ephemerides = load_ephemeris(&(filename.to_string() + ".exb"));
        for ephem in ephemerides {
            let id = ephem.id.clone().unwrap();
            cosm.ephem_map.insert((id.number, id.name), ephem);
        }

        let frames = load_frames(&(filename.to_string() + ".fxb"));
        for frame in frames {
            let id = frame.id.clone().unwrap();
            cosm.frame_map.insert((id.number, id.name), frame);
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
