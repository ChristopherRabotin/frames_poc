// Defines Cosm, from the Greek word for "world" or "universe".

extern crate petgraph;

use petgraph::Graph;

pub trait Cosm {}

#[derive(Debug)]
pub struct GraphMemCosm {
    graph: Graph<i32, i32>,
}

impl Default for GraphMemCosm {
    fn default() -> Self {
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
