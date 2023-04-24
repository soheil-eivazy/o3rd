use std::collections::HashMap;

use crate::maths::graph::{Node, GraphSearchMethod, SearchResult, BreadthFirst, GraphSearch};
use crate::maze_runner::{plane::generate_graph, generator::division};



pub struct Maze {
    plane: Vec<Vec<u8>>,
    runner: Box<dyn GraphSearch>,
    paths: HashMap<Node, Vec<Node>>,
}

impl Maze {
    pub fn new(width: usize, height: usize, search_method: GraphSearchMethod) -> Self {
        let plane = division(width, height);
        let graph = generate_graph(&plane);

        let runner = match search_method {
            GraphSearchMethod::AStar => {BreadthFirst::new(graph)},
            GraphSearchMethod::BreadthFirst => {BreadthFirst::new(graph)},
            GraphSearchMethod::DepthFirst => {BreadthFirst::new(graph)},
        };

        Maze { 
            plane, 
            runner: Box::new(runner),
            paths: HashMap::new(), 
        }
    }

    pub fn run(&mut self) -> Vec<u32>  {
        let res = Vec::new();
        let mut blocked: Vec<Node> = vec![];


        match self.runner.next() {
            SearchResult::Done => {res},
            SearchResult::Failed => {res},
            SearchResult::Next(data) => {
                for (k, v) in data.into_iter() {
                    if v.len() == 0 {
                        if self.paths.contains_key(&k) {
                            self.paths.remove(&k);
                        } 
                        blocked.push(k);

                    } else {
                        self.paths.insert(k, v);
                    }
                }

                self.flat_path(blocked)
            },
        }
    }

    fn flat_path(&mut self, to_remove: Vec<Node>) -> Vec<u32> {
        let mut res = Vec::new();
        let mut new_path = HashMap::<Node, Vec<Node>>::new();
        
        for (k, v) in self.paths.clone().into_iter() {
            let mut edges: Vec<Node> = Vec::new();
            for node in v.iter() {
                if !to_remove.contains(node) {
                    edges.push(*node);
                    res.push(k.x);
                    res.push(k.y);
                    res.push(node.x);
                    res.push(node.y);    
                }
            }

            if edges.len() > 0 {
                new_path.insert(k, edges);
            }
        }

        self.paths = new_path;

        res
    }
}

