use std::collections::{HashMap, hash_map::Entry};




#[derive(Debug)]
pub struct Graph {
    nodes: HashMap<String, Node>,
    edges: HashMap<String, Vec<String>>,
}


impl Graph {
    pub fn new() -> Graph {
        Graph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) -> bool {
        match self.nodes.entry(node.id()) {
            Entry::Occupied(_) => {true},
            Entry::Vacant(v) => {
                v.insert(node);
                false
            },
        }
    }

    pub fn add_edge(&mut self, edge: (String, String)) {

        self.edges.entry(edge.0.clone())
            .and_modify(|e| e.push(edge.1.clone()))
            .or_insert_with(|| vec![edge.1.clone()]);

        self.edges.entry(edge.1)
            .and_modify(|e| e.push(edge.0.clone()))
            .or_insert_with(|| vec![edge.0]);
    } 

}


#[derive(Clone, Debug)]
pub struct Node {
    x: i32,
    y: i32,
}

impl Node {
    pub fn new(x: i32, y: i32) -> Node {
        Node {x, y}
    }

    pub fn id(&self) -> String {
        return format!("{}-{}", self.x, self.y)
    }
}