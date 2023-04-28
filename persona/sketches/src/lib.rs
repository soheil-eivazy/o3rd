pub mod maths;
pub mod maze_runner;

use maths::graph::{Graph, Node};
use wasm_bindgen::prelude::*;
// 

#[wasm_bindgen]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}









pub fn graphing() {
    let mut graph = Graph::new();
    let node1 = Node::new(1, 1);
    let node2 = Node::new(1, 2);
    let node3 = Node::new(2, 1);
    let node4 = Node::new(2, 2);

    let edge1 = (node1, node2);
    let edge2 = (node1, node3);
    let edge3 = (node1, node4);
    let edge4 = (node2, node4);

    graph.add_node(node1);
    graph.add_node(node2);
    graph.add_node(node3);
    graph.add_node(node4);

    graph.add_edge(edge1);
    graph.add_edge(edge2);
    graph.add_edge(edge3);
    graph.add_edge(edge4);


    dbg!(graph);

}


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    #[test]
    fn it_works() {
        let mut maze = maze_runner::maze::Maze::new(10, 10, maths::graph::GraphSearchMethod::BreadthFirst);
    
        let mut done = false;
        while !done {
            let res = maze.run();
            // dbg!(&res);

            if res.len() == 0 {
                done = true;
            }
        }
    }


    #[test]
    fn vec_test() {
        let mut data = VecDeque::<i32>::new();

        for i in 0..15 {
            data.push_back(i);
        }
        
        dbg!(data.len());

        for _ in 0..5 {
            let a =  data.pop_front();
            dbg!(a);
        }

        dbg!(data.len());
       
    }
}
