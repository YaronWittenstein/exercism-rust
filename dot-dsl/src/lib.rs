pub mod graph {
    use self::graph_items::edge::Edge;
    use self::graph_items::node::Node;
    use std::collections::HashMap;
    use std::default::Default;

    pub mod graph_items {
        pub mod node {
            use std::collections::HashMap;

            #[derive(Debug, Clone, PartialEq)]
            pub struct Node {
                pub label: String,
                pub attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(label: &str) -> Self {
                    Self {
                        label: label.to_string(),
                        attrs: Default::default(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for (k, v) in attrs {
                        self.attrs.insert(k.to_string(), v.to_string());
                    }

                    self
                }

                pub fn get_attr(&self, attr: &str) -> Option<&str> {
                    let v = self.attrs.get(attr);

                    match v {
                        None => None,
                        Some(value) => Some(&value),
                    }
                }
            }
        }

        pub mod edge {
            use std::collections::HashMap;

            #[derive(Debug, Clone, PartialEq)]
            pub struct Edge {
                pub from: String,
                pub to: String,
                pub attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(from: &str, to: &str) -> Self {
                    Self {
                        from: from.to_string(),
                        to: to.to_string(),
                        attrs: Default::default(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for (k, v) in attrs {
                        self.attrs.insert(k.to_string(), v.to_string());
                    }

                    self
                }
            }
        }
    }

    #[derive(Debug)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self {
                nodes: Default::default(),
                edges: Default::default(),
                attrs: Default::default(),
            }
        }

        pub fn get_node(&self, label: &str) -> Option<&Node> {
            self.nodes.iter().find(|node| node.label == *label)
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes = nodes.to_vec();
            self
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
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
