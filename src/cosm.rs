// Defines Cosm, from the Greek word for "world" or "universe".

extern crate petgraph;

use crate::frames::*;
use nalgebra::UnitQuaternion;
use petgraph::Graph;
use std::collections::HashMap;

pub trait Cosm {}

#[derive(Debug)]
pub struct GraphMemCosm {
    graph: Graph<Identifier, i32>,
}

impl Default for GraphMemCosm {
    fn default() -> Self {
        // Start by building all the celestial bodies
        let ssb = Geoid::perfect_sphere(
            Identifier {
                number: 0,
                name: "SSB".to_string(),
            },
            1.32712440018e20,
        );

        let mercury = Geoid::perfect_sphere(
            Identifier {
                number: 1,
                name: "Mercrury".to_string(),
            },
            22_032.080_486_418,
        );

        let venus = Geoid::perfect_sphere(
            Identifier {
                number: 2,
                name: "Venus".to_string(),
            },
            324_858.598_826_46,
        );

        let earth = Geoid::perfect_sphere(
            Identifier {
                number: 3,
                name: "Earth".to_string(),
            },
            398_600.441_5,
        );

        // Then build all the frames
        let ssbj2k = Frame {
            center: ssb,
            xb_id: Identifier {
                number: 100,
                name: "SSB J2000".to_string(),
            },
        };

        let eci = Frame {
            center: earth.clone(),
            xb_id: Identifier {
                number: 300,
                name: "Earth Centered Inertial J2000".to_string(),
            },
        };

        let ecef = Frame {
            center: earth,
            xb_id: Identifier {
                number: 310,
                name: "Earth Centered Earth Fixed J2000".to_string(),
            },
        };

        // And now set everything in the graph
        let mut og = Graph::new();
        let ssbj2k_e = og.add_node(ssbj2k.xb_id);
        let eci_e = og.add_node(eci.xb_id);
        let ecef_e = og.add_node(ecef.xb_id);
        og.add_edge(ssbj2k_e, eci_e, 2);
        og.add_edge(eci_e, ecef_e, 1);
        GraphMemCosm { graph: og }
    }
}
