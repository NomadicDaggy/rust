// nodes are strings
// edges are pairs of strings (tuples)
// attrs are also pairs of strings but are specified as list of tuples
// both edges and nodes have attrs
// graph has with_nodes(), with_edges() and with_attrs()
// nodes, edges and attrs have is_empty()
// attrs is a HashMap

pub mod graph {
    pub struct Graph {
        nodes: Node,
        edges: Edge,
        attrs: list,
    }

    impl Graph {
        pub fn new() -> Self {
            unimplemented!("Construct a new Graph struct.");
        }
    }
}
