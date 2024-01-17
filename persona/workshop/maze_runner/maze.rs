use wasm_bindgen::prelude::*;
use crate::maths::graph::{Node, GraphSearchMethod, SearchResult, BreadthFirst, Graph, GraphSearch};
use super::{plane::generate_graph, generator::division};

#[wasm_bindgen]
pub struct Maze {
    plane: Vec<Vec<u8>>,
    runner: Box<dyn GraphSearch>,
    // paths: Vec<(Node, Node)>,
    first_last: (Node, Node),
    graph: Graph,
}

#[wasm_bindgen]
impl Maze {
    pub fn new(width: usize, height: usize, search_method: GraphSearchMethod) -> Self {
        let plane = division(width, height);
        let graph = generate_graph(&plane);
        let first_last = graph.first_and_last();

        let runner = match search_method {
            GraphSearchMethod::AStar => {BreadthFirst::new(graph.clone())},
            GraphSearchMethod::BreadthFirst => {BreadthFirst::new(graph.clone())},
            GraphSearchMethod::DepthFirst => {BreadthFirst::new(graph.clone())},
        };

        Maze { 
            plane, 
            runner: Box::new(runner),
            // paths: Vec::new(), 
            first_last,
            graph,
        }
    }

    pub fn first_and_last(&self) -> Vec<u16> {
        vec![
            self.first_last.0.x,
            self.first_last.0.y,
            self.first_last.1.x,
            self.first_last.1.y
        ]
    }

    pub fn all_edges(&self) -> Vec<u16> {
        let mut res = vec![];

        for (k, v) in self.graph.edges.iter() {
            for node in v.iter() {
                res.push(k.x);
                res.push(k.y);
                res.push(node.x);
                res.push(node.y); 
            }
        }

        res
    }

    pub fn run(&mut self) -> Vec<u16>  {
        let mut res = Vec::new();

        match self.runner.next() {
            SearchResult::Failed => {res},
            SearchResult::Done(solved) => {
                dbg!(&solved);
                res.push(1);
                for n1 in solved.iter() {
                    res.push(n1.x);
                    res.push(n1.y);
                }

                res
            },
            SearchResult::Next(edges) => {
                res.push(0);
                for (k, v) in edges.into_iter() {
                    for n1 in v.iter() {
                        res.push(k.x);
                        res.push(k.y);
                        res.push(n1.x);
                        res.push(n1.y);
                    }
                }

                res   
            },
        }
    }

    pub fn flat_plane(&self) -> Vec<u8> {

        let mut flat = Vec::new();

        for row in self.plane.iter() {
            flat.extend(row);
        }

        return flat
    }
}

