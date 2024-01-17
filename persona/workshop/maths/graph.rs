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
            let node =  self.current.pop_front().unwrap();
            if node.node == self.last {
                let mut parent = node.parent;
                let mut has_parent = true;
                let mut solved = Vec::<Node>::new();
                solved.push(node.node);
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



// pub struct DepthFirst {
//     graph: Graph,
//     first: Node,
//     last: Node,
//     current: VecDeque<Path>,
//     beaten_path: HashMap<Node, Node>,
//     count: u32,
// }

// impl DepthFirst {
//     pub fn new(graph:Graph) -> Self {
//         let (first, last) = graph.first_and_last();
//         let mut current = VecDeque::new();
//         current.push_back(Path{node:first, parent:None});
//         dbg!(graph.edges.len());
//         DepthFirst { 
//             graph,
//             first,
//             last,
//             current,
//             beaten_path: HashMap::new(),
//             count: 0,
//         }
//     }
// }


// impl GraphSearch for DepthFirst {
//     fn next(&mut self) -> SearchResult {
//         // procedure DFS_iterative(G, v) is
//         //     let S be a stack
//         //     label v as discovered
//         //     S.push(iterator of G.adjacentEdges(v))
//         //     while S is not empty do
//         //         if S.peek().hasNext() then
//         //             w = S.peek().next()
//         //             if w is not labeled as discovered then
//         //                 label w as discovered
//         //                 S.push(iterator of G.adjacentEdges(w))
//         //         else
//         //             S.pop()

//         SearchResult::Failed
//     }
// }

// pub struct AStar {
//     graph: Graph,
//     first: Node,
//     last: Node,
//     current: VecDeque<Path>,
//     beaten_path: HashMap<Node, Node>,
//     count: u32,
// }

// impl AStar {
//     pub fn new(graph:Graph) -> Self {
//         let (first, last) = graph.first_and_last();
//         let mut current = VecDeque::new();
//         current.push_back(Path{node:first, parent:None});
//         dbg!(graph.edges.len());
//         AStar { 
//             graph,
//             first,
//             last,
//             current,
//             beaten_path: HashMap::new(),
//             count: 0,
//         }
//     }
// }


// impl GraphSearch for AStar {
//     fn next(&mut self) -> SearchResult {
//     // function reconstruct_path(cameFrom, current)
//     //     total_path := {current}
//     //     while current in cameFrom.Keys:
//     //         current := cameFrom[current]
//     //         total_path.prepend(current)
//     //     return total_path

//     // // A* finds a path from start to goal.
//     // // h is the heuristic function. h(n) estimates the cost to reach goal from node n.
//     // function A_Star(start, goal, h)
//     //     // The set of discovered nodes that may need to be (re-)expanded.
//     //     // Initially, only the start node is known.
//     //     // This is usually implemented as a min-heap or priority queue rather than a hash-set.
//     //     openSet := {start}

//     //     // For node n, cameFrom[n] is the node immediately preceding it on the cheapest path from the start
//     //     // to n currently known.
//     //     cameFrom := an empty map

//     //     // For node n, gScore[n] is the cost of the cheapest path from start to n currently known.
//     //     gScore := map with default value of Infinity
//     //     gScore[start] := 0

//     //     // For node n, fScore[n] := gScore[n] + h(n). fScore[n] represents our current best guess as to
//     //     // how cheap a path could be from start to finish if it goes through n.
//     //     fScore := map with default value of Infinity
//     //     fScore[start] := h(start)

//     //     while openSet is not empty
//     //         // This operation can occur in O(Log(N)) time if openSet is a min-heap or a priority queue
//     //         current := the node in openSet having the lowest fScore[] value
//     //         if current = goal
//     //             return reconstruct_path(cameFrom, current)

//     //         openSet.Remove(current)
//     //         for each neighbor of current
//     //             // d(current,neighbor) is the weight of the edge from current to neighbor
//     //             // tentative_gScore is the distance from start to the neighbor through current
//     //             tentative_gScore := gScore[current] + d(current, neighbor)
//     //             if tentative_gScore < gScore[neighbor]
//     //                 // This path to neighbor is better than any previous one. Record it!
//     //                 cameFrom[neighbor] := current
//     //                 gScore[neighbor] := tentative_gScore
//     //                 fScore[neighbor] := tentative_gScore + h(neighbor)
//     //                 if neighbor not in openSet
//     //                     openSet.add(neighbor)

//     //     // Open set is empty but goal was never reached
//     //     return failure

//         SearchResult::Failed
//     }
// }