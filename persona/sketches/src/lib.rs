pub mod maths;
pub mod maze_runner;

use maths::graph::{Graph, Node};
use wasm_bindgen::prelude::*;


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

    let edge1 = (node1.id(), node2.id());
    let edge2 = (node1.id(), node3.id());
    let edge3 = (node1.id(), node4.id());
    let edge4 = (node2.id(), node4.id());

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

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
