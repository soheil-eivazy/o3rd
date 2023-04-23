use crate::maths::graph::{Graph, Node, self};
use crate::maze_runner::{plane::generate_graph, generator::division};



trait MazeRunner {
    fn load_flat_plane(&self) -> Vec<u8>;
    fn load_path(&self) -> Vec<u8>;
}


struct Maze {
    plane: Vec<Vec<u8>>,
    graph: Graph,
}

impl Maze {
    fn new(width: usize, height: usize) -> Self {
        let plane = division(width, height);
        let graph = generate_graph(&plane);

        Maze { plane, graph}
    }
}

pub struct BreadthFirst {
    maze: Maze
}


impl BreadthFirst {
    pub fn new(width: usize, height: usize) -> Self {
        BreadthFirst { maze: Maze::new(width, height) }
    }
}

impl MazeRunner for BreadthFirst {
    fn load_flat_plane(&self) -> Vec<u8> {
        Vec::new()
    }

    fn load_path(&self) -> Vec<u8> {
        Vec::new()
    }
}