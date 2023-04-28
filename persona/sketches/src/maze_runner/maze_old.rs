// use wasm_bindgen::prelude::*;
use crate::maze_runner::generator::{division};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}



// #[wasm_bindgen]
struct Maze {
    width: usize,
    height: usize,
    grid: Vec<u8>,
    _matrix: Vec<Vec<u8>>,
}


impl Maze {
    fn new(width: usize, height: usize) -> Self {
        let _matrix = division(width, height);
        Maze { 
            width,
            height,
            grid: _matrix.clone().into_iter().flatten().collect(),
            _matrix,
        }
    }

    fn _repaint(&mut self) {
        self.grid = self._matrix.clone().into_iter().flatten().collect();
    }

    fn get_index(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }

    fn graph(&self) -> Vec<Vec<Vec<Point>>> {
        let mut grid: Vec<Vec<Vec<Point>>> = Vec::new();

        let mut new_row: Vec<Vec<Point>> = Vec::new();
        for (i, cell) in self.grid.iter().enumerate() {
            
            if i > 0 && i % self.width == 0 {
                grid.push(new_row);
                new_row = Vec::new();
            }

            new_row.push(
                if cell == &0 {
                    Vec::<Point>::new()
                } else {
                    self.neighbors(i % (self.width), i / (self.width))
                }    
            )
        }

        grid
    }

    fn neighbors(&self, xp: usize, yp: usize) -> Vec<Point> {
        let mut ns: Vec<Point> = Vec::new();

        for y in [yp-1, yp+1] {
            // println!("y:{}, x:{}, p:{}", y, xp, self.grid[y][xp]);
            if self.grid[self.get_index(y, xp)] == 1 {
                ns.push(Point{x: xp, y})
            }
        }

        for x in [xp-1, xp+1] {
            // println!("y:{}, x:{}, p:{}", yp, x, self.grid[yp][x]);
            if self.grid[self.get_index(yp, x)] == 1 {
                ns.push(Point{x, y:yp})
            }
        }

        ns
    }

    fn start(&mut self) -> Option<Point> {
        for i in (self.width+1)..(2*(self.width)) {
            if self.grid[i] == 1 {
                self.grid[i] = 2;
                return Some(Point{x: i%(self.width), y:1})
            }
        }

        None
    }

    fn end(&mut self) -> Option<Point> {
        let l = self.grid.len();
        for i in ((l - (2 * self.width))..(l - self.width)).collect::<Vec<usize>>().iter().rev() {
            if self.grid[*i] == 1 {
                self.grid[*i] = 2;
                return Some(Point{x:i%(self.width), y:self.height-2})
            }
        }

        None
    }

    fn visited(&mut self, point: &Point) {
        // for p in points {
            self.grid[point.y * self.width + point.x] = 3
        // }
    }

    fn stepping(&mut self, point: &Point) {
        // for p in points {
            self.grid[point.y * self.width + point.x] = 2
        // }
    }

    fn running(&mut self, points: &Vec<Point>, c: u8) {
        for point in points {
            self.grid[point.y * self.width + point.x] = c
        }
    }
}

#[derive(Debug, Clone)]
struct BFRunner {
    current: Point,
    last: Point,
    steps: Vec<Point>,
}

impl BFRunner {
    fn new(start: Point) -> Self {
        BFRunner {
            current: start, 
            last: start, 
            steps: Vec::from([start])
        }
    }
}


// #[wasm_bindgen]
pub struct BreadthFirst {
    maze: Maze,
    graph: Vec<Vec<Vec<Point>>>,
    end: Point,
    runners: Vec<BFRunner>,
    finished: bool
}


// #[wasm_bindgen]
impl BreadthFirst {
    pub fn new(width: usize, height: usize) -> Self {
        let mut maze = Maze::new(width, height);
        BreadthFirst {
            graph: maze.graph(),
            end: maze.end().unwrap(),
            finished: false,
            runners: Vec::from([BFRunner::new(maze.start().unwrap())]),
            maze,
        }
    }


    pub fn run_better(&mut self) {
        if self.finished {return}

        for runner in self.runners.iter_mut() {

        } 
    }


    pub fn run(&mut self, c: u8) {
        // horrible execution!
        if self.finished {return}

        let mut new_runners: Vec<BFRunner> = Vec::new();

        for runner in self.runners.iter() {

            if runner.current == self.end {
                new_runners = Vec::new();
                new_runners.push(runner.clone());
                self.finished = true;
                
                break;
            }
            
            let step = &self.graph[runner.current.y][runner.current.x];

            for s in step {
                if !runner.steps.contains(s) {
                    let mut nr = BFRunner {
                        last: runner.current,
                        current: *s,
                        steps: runner.steps.clone(),
                    };

                    nr.steps.push(*s);
                    new_runners.push(nr);
                }
            }
        }

        for runner in new_runners.iter() {
            self.maze.running(&runner.steps, c);
        }

        self.runners = new_runners;
    }

    pub fn cells(&self) -> *const u8 {
        self.maze.grid.as_ptr()
    }

    pub fn is_done(&self) -> bool {
        self.finished
    }

    pub fn get_end(&self) -> Vec<usize> {
        vec![self.end.x, self.end.y]
    }
}



// struct BreadthFirstNext {
//     maze: Maze,
//     runners: Vec<BFRunner>,
//     end: Point,
//     is_done: bool
// }

// impl BreadthFirstNext {
//     fn run(&mut self) {
//         for runner in self.runners.iter_mut() {
//             let steps = self.maze.neighbors(runner.current.x, runner.current.y);


//         }
//     }
// }







struct Node {
    index: usize,
    coords: Point
}


// #[wasm_bindgen]
pub struct DepthFirst {
    maze: Maze,
    graph: Vec<Vec<Vec<Point>>>,
    end: Point,
    nodes: Vec<Node>,
    runner: Vec<Point>,
    finished: bool,
}


// #[wasm_bindgen]
impl DepthFirst {
    pub fn new(width: usize, height: usize) -> Self {
        let mut maze = Maze::new(width, height);

        DepthFirst {
            graph: maze.graph(),
            end: maze.end().unwrap(),
            finished: false,
            runner: vec![maze.start().unwrap()],
            nodes: Vec::new(),
            maze
        }
    }

    pub fn run(&mut self) {
        if self.finished {return}

        let current = self.runner.last().unwrap();

        if current == &self.end {
            self.finished = true;
            return
        }

        let steps = &self.graph[current.y][current.x];
        for step in steps {
            if !self.runner.contains(step) {
                self.nodes.push(Node {index: self.runner.len(), coords: *step})
            }
        }

        let next_node = self.nodes.pop().unwrap();

        for _ in next_node.index..self.runner.len() {
            if let Some(point) = self.runner.pop() {
                self.maze.visited(&point);
            }
        }

        self.maze.stepping(&next_node.coords);
        self.runner.push(next_node.coords);
    }

    pub fn cells(&self) -> *const u8 {
        self.maze.grid.as_ptr()
    }

    pub fn is_done(&self) -> bool {
        self.finished
    }

    pub fn get_end(&self) -> Vec<usize> {
        vec![self.end.x, self.end.y]
    }
}