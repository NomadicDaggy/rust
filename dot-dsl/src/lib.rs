// nodes are strings
// edges are pairs of strings (tuples)
// attrs are also pairs of strings but are specified as list of tuples
// both edges and nodes have attrs
// graph has with_nodes(), with_edges() and with_attrs()
// nodes, edges and attrs have is_empty()
// attrs is a HashMap


pub mod graph {
    use maplit::hashmap;
    use std::collections::HashMap;

    pub struct Graph {
        nodes: Vec<Node>,
        edges: Vec<Edge>,
        attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: Vec::<Node>::new(),
                edges: Vec::<Edge>::new(),
                attrs: hashmap![],
            }
        }
    }

    struct Edge {
        from: String,
        to: String,
        attrs: HashMap<String, String>,
    }

    struct Node {
        name: String,
        attrs: HashMap<String, String>,
    }

    //impl Node {
    //    fn new()
    //}
}
