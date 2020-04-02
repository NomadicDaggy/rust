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

        pub fn with_nodes<'a>(&'a mut self, nodes: Vec<Node>) -> &'a mut Graph {
            self.nodes.extend(nodes);
            self
        }

        pub fn with_edges<'a>(&'a mut self, edges: Vec<Edge>) -> &'a mut Graph {
            self.edges.extend(edges);
            self
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct Edge {
        from: String,
        to: String,
        attrs: HashMap<String, String>,
    }

    #[derive(Debug, PartialEq)]
    pub struct Node {
        name: String,
        attrs: HashMap<String, String>,
    }

    //impl Node {
    //    fn new()
    //}
}
