use std::collections::{HashMap, VecDeque};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Node {
    pub x: u32,
    pub y: u32,
}

impl Node {
    pub fn new(x: u32, y: u32) -> Node {
        Node {x, y}
    }

    // pub fn id(&self) -> (u32, u32) {
    //     (self.x, self.y)
    // }
}


#[derive(Debug)]
pub struct Graph {
    nodes: Vec<Node>,
    edges: HashMap<Node, Vec<Node>>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            nodes: Vec::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) -> bool {
        if !self.nodes.contains(&node) {
            self.nodes.push(node);
            return false
        }

        true
        // match self.nodes.entry(node.id()) {
        //     Entry::Occupied(_) => {true},
        //     Entry::Vacant(v) => {
        //         v.insert(node);
        //         false
        //     },
        // }
    }

    pub fn add_edge(&mut self, edge: (Node, Node)) {

        self.edges.entry(edge.0)
            .and_modify(|e| e.push(edge.1.clone()))
            .or_insert_with(|| vec![edge.1.clone()]);

        self.edges.entry(edge.1)
            .and_modify(|e| e.push(edge.0.clone()))
            .or_insert_with(|| vec![edge.0]);
    }

    pub fn first_and_last(&self) -> (Node, Node) {
        // let mut node_list: Vec<&Node> = self.nodes;
        let mut res = self.nodes.clone();
        res.sort();

        (
            res.first().unwrap().to_owned(), 
            res.last().unwrap().to_owned()
        )
    }


    fn load_edges(&self, node: &Node) -> Vec<Node> {
        self.edges.get(node).unwrap_or(&Vec::new()).clone()
    }


    
}



// pub fn setup_iterator(&mut self, method: SearchMethod) {
    //     self.search_method = 
    // } 

pub enum GraphSearchMethod {
    BreadthFirst,
    DepthFirst,
    AStar,
}

pub trait GraphSearch {
    fn next(&mut self) -> SearchResult;
}


pub enum SearchResult {
    Done,
    Failed,
    Next(HashMap<Node, Vec<Node>>),
}

pub struct BreadthFirst {
    graph: Graph,
    last: Node,
    current: VecDeque<Node>,
    beaten_path: Vec<Node>,
}


impl BreadthFirst {
    pub fn new(graph:Graph) -> Self {
        let (first, last) = graph.first_and_last();
        let mut current = VecDeque::new();
        current.push_back(first);
        BreadthFirst { 
            graph,
            last,
            current,
            beaten_path: Vec::new(),
        }
    }
}

impl GraphSearch for BreadthFirst {
    fn next(&mut self) -> SearchResult {

        let cur_len = self.current.len();
        let mut result = HashMap::new();

        for _ in 0..cur_len {
            let node = self.current.pop_front().unwrap();
            if node == self.last {
                return SearchResult::Done
            }

            let mut edges = Vec::new();
            for edge in self.graph.load_edges(&node) {
                if !self.beaten_path.contains(&edge) {
                    edges.push(edge)
                }
            }

            result.insert(node.to_owned(), edges.clone());
            self.current.extend(edges);
            self.beaten_path.push(node.to_owned());
        }

        if self.current.is_empty() {
            return SearchResult::Failed
        }

        SearchResult::Next(result)
    }
}