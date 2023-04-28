pub mod maths;
pub mod maze_runner;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(left: usize, right: usize) -> usize {
    left + right
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
