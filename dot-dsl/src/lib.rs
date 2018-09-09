pub mod graph {
    use std::collections::HashMap;

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Debug, Clone)]
            pub struct Edge {
                from: String,
                to: String,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(from: &str, to: &str) -> Edge {
                    Edge {
                        from: from.to_string(),
                        to: to.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for (k, v) in attrs {
                        self.attrs.insert(k.to_string(), v.to_string());
                    }
                    self
                }
            }

            impl PartialEq for Edge {
                fn eq(&self, other: &Edge) -> bool {
                    self.from == other.from && self.to == other.to && self.attrs == other.attrs
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(Debug, Clone)]
            pub struct Node {
                label: String,
                attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(label: &str) -> Node {
                    Node {
                        label: label.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for (k, v) in attrs {
                        self.attrs.insert(k.to_string(), v.to_string());
                    }
                    self
                }
            }

            impl PartialEq for Node {
                fn eq(&self, other: &Node) -> bool {
                    self.label == other.label && self.attrs == other.attrs
                }
            }
        }
    }

    pub struct Graph {
        pub nodes: Vec<graph_items::node::Node>,
        pub edges: Vec<graph_items::edge::Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Graph {
            Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(mut self, nodes: &Vec<graph_items::node::Node>) -> Self {
            self.nodes = nodes.to_vec();
            self
        }

        pub fn with_edges(mut self, edges: &Vec<graph_items::edge::Edge>) -> Self {
            self.edges = edges.to_vec();
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            for (k, v) in attrs {
                self.attrs.insert(k.to_string(), v.to_string());
            }
            self
        }
    }
}
