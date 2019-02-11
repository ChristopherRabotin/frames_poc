// Defines Cosm, from the Greek word for "world" or "universe".

extern crate hashbrown;
extern crate petgraph;

use crate::frames::{CelestialBody, CelestialFrame};
use hashbrown::HashMap;
use nalgebra::UnitQuaternion;
use petgraph::Graph;

pub trait Cosm {}

#[derive(Debug)]
pub struct GraphMemCosm {
    graph: Graph<i32, i32>,
}

impl Default for GraphMemCosm {
    fn default() -> Self {
        // Start by building all the celestial bodies
        let ssb = CelestialBody {
            gm: -1.0, // BUG: Wrong value
            name: "Solar System Barycenter".to_string(),
        };
        let mercury = CelestialBody {
            gm: 22_032.080_486_418,
            name: "Mercury".to_string(),
        };
        let venus = CelestialBody {
            gm: 324_858.598_826_46,
            name: "Venus".to_string(),
        };
        let earth = CelestialBody {
            gm: 398_600.441_5,
            name: "Earth".to_string(),
        };
        // Then build all the frames
        let ssbj2k = CelestialFrame {
            parent: None,
            rotation: UnitQuaternion::identity(),
            body: Box::new(ssb),
        };
        let eci = CelestialFrame {
            parent: Some(Box::new(ssbj2k)),
            rotation: UnitQuaternion::identity(),
            body: &earth,
        };

        // BUG: The rotation implementation is here defined in frames.rs
        // This is a problem because in the case of ECEF, that rotation changes
        // with time! Hence I think there is a need for different kinds of rotations
        // similar to how there are different "tracking" types in AXB.
        // Either a fixed rotation, an interpolated rotation, or a constant angular velocity,
        // or a combination of these depending on the time (e.g. camera slews for X seconds
        // and is then fixed).
        let ecef = CelestialFrame {
            parent: Some(Box::new(eci)),
            rotation: UnitQuaternion::identity(),
            body: &earth,
        };
        // And now set everything in the graph
        let mut og = Graph::new();
        let a = og.add_node(0);
        let b = og.add_node(1);
        let c = og.add_node(2);
        let d = og.add_node(3);
        let _ = og.add_edge(a, b, 0);
        let _ = og.add_edge(a, c, 1);
        og.add_edge(c, a, 2);
        og.add_edge(a, a, 3);
        og.add_edge(b, c, 4);
        og.add_edge(b, a, 5);
        og.add_edge(a, d, 6);
        GraphMemCosm { graph: og }
    }
}
