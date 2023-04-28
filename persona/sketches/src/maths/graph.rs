use std::collections::{HashMap, VecDeque};
use wasm_bindgen::prelude::*;
// use std::time::{Duration, Instant};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Node {
    pub x: u16,
    pub y: u16,
}

impl Node {
    pub fn new(x: u16, y: u16) -> Node {
        Node {x, y}
    }
}


#[derive(Debug, Clone)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: HashMap<Node, Vec<Node>>,
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
        let mut res = self.nodes.clone();
        res.sort_unstable_by_key(|item| (item.y, item.x));

        (res.first().unwrap().to_owned(), res.last().unwrap().to_owned())
    }


    fn load_edges(&self, node: &Node) -> Vec<Node> {
        self.edges.get(node).unwrap_or(&Vec::new()).clone()
    }
}

#[wasm_bindgen]
pub enum GraphSearchMethod {
    BreadthFirst,
    DepthFirst,
    AStar,
}

pub trait GraphSearch {
    fn next(&mut self) -> SearchResult;
}

pub enum SearchResult {
    Done(Vec<Node>),
    Failed,
    Next(HashMap<Node, Vec<Node>>),
}

#[derive(Copy, Clone, Debug, Hash, PartialOrd)]
struct Path {
    node: Node,
    parent: Option<Node>
}

impl PartialEq for Path {
    fn eq(&self, node: &Path) -> bool {
        self.node == node.node
    }

    fn ne(&self, node: &Path) -> bool {
        self.node != node.node
    }
}

pub struct BreadthFirst {
    graph: Graph,
    first: Node,
    last: Node,
    current: VecDeque<Path>,
    beaten_path: HashMap<Node, Node>,
    count: u32,
}

impl BreadthFirst {
    pub fn new(graph:Graph) -> Self {
        let (first, last) = graph.first_and_last();
        let mut current = VecDeque::new();
        current.push_back(Path{node:first, parent:None});
        dbg!(graph.edges.len());
        BreadthFirst { 
            graph,
            first,
            last,
            current,
            beaten_path: HashMap::new(),
            count: 0,
        }
    }
}

impl GraphSearch for BreadthFirst {
    fn next(&mut self) -> SearchResult {
        self.count += 1;
        // let start = Instant::now();

        let cur_len = self.current.len();
        let mut result = HashMap::new();

        for _ in 0..cur_len {
            // dbg!(self.current.len());
            let node =  self.current.pop_front().unwrap();
            // dbg!(self.current.len());

            if node.node == self.last {
                let mut parent = node.parent;
                let mut has_parent = true;
                let mut solved = Vec::<Node>::new();
                solved.push(node.node);
                // dbg!(&self.beaten_path);
                while has_parent {
                    solved.push(parent.unwrap());
                    if let Some(p) = self.beaten_path.get(&parent.unwrap()) {
                        parent = Some(*p);

                        if *p == self.first {
                            solved.push(parent.unwrap());
                            has_parent = false;
                        }

                    } else {
                        has_parent = false;
                    }

                    
                }

                return SearchResult::Done(solved);
            }

            let mut n = Vec::new();
            for edge in self.graph.load_edges(&node.node) {
                
                if !self.beaten_path.contains_key(&edge) {
                    n.push(edge);
                    self.current.push_back(Path{node: edge, parent: Some(node.node)});
                    self.beaten_path.insert(edge, node.node);
                } 
            }

            result.insert(node.node, n);
        }

        if self.current.is_empty() {
            return SearchResult::Failed
        }

        // let duration = start.elapsed();
        // dbg!(self.count, cur_len, self.beaten_path.len(), duration);

        SearchResult::Next(result)
    }
}