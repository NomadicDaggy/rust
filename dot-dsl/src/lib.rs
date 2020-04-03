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
    
    use graph_items::edge::Edge;
    use graph_items::node::Node;

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: Vec::<Node>::new(),
                edges: Vec::<Edge>::new(),
                attrs: hashmap![],
            }
        }

        pub fn with_nodes<'a>(&'a mut self, nodes: &[Node]) -> &'a mut Graph {
            self.nodes = nodes.to_vec();
            self
        }

        pub fn with_edges<'a>(&'a mut self, edges: &[Edge]) -> &'a mut Graph {
            self.edges = edges.to_vec();
            self
        }

        pub fn get_node(&self, name: &str) -> Option<&Node> {
            self.nodes.iter().find(|node| node.name == name)
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs = attrs
                .iter()
                .map(|(a, b)| (a.to_string(), b.to_string()))
                .collect();
            self
        }
    }

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;
            use maplit::hashmap;

            #[derive(Debug, PartialEq, Clone)]
            pub struct Edge {
                from: String,
                to: String,
                attrs: HashMap<String, String>,
            }
    
            impl Edge {
                pub fn new(from: &str, to: &str) -> Self {
                    Edge {
                        from: from.to_string(),
                        to: to.to_string(),
                        attrs: hashmap![],
                    }
                }

                // cheated with this
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs = attrs
                        .iter()
                        .map(|(a, b)| (a.to_string(), b.to_string()))
                        .collect();
                    self
                }
            }
        }
        
        pub mod node {
            use std::collections::HashMap;
            use maplit::hashmap;

            #[derive(Debug, PartialEq, Clone)]
            pub struct Node {
                pub name: String,
                attrs: HashMap<String, String>,
            }
    
            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name: name.to_string(),
                        attrs: hashmap![],
                    }
                }

                pub fn get_attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|s| s.as_str())
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs = attrs
                        .iter()
                        .map(|(a, b)| (a.to_string(), b.to_string()))
                        .collect();
                    self
                }
            }
        }
    }
}
