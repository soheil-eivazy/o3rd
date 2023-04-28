use crate::maths::graph::{Graph, Node};


pub fn generate_graph(plane: &Vec<Vec<u8>>) -> Graph {
    let mut graph = Graph::new();
    for (y, row) in plane.iter().enumerate() {
        for (x, b) in row.iter().enumerate() {

            if *b != 1 { continue;}

            let ways = (plane[y-1][x] * 1) + (plane[y][x+1] * 2) + (plane[y+1][x] * 4) + (plane[y][x-1] * 8);
            if [5, 10].contains(&ways) { continue;}

            let node = Node::new(x as u16, y as u16);
            graph.add_node(node);

            let mut ty = y + 1;
            while ty < plane.len() {
                if add_edge(&mut graph, node, plane, x, ty) {
                    break;
                }

                ty += 1;
            }

            if y > 0 {
                let mut ty = y - 1;
                while ty > 0 {
                    if add_edge(&mut graph, node, plane, x, ty) {
                        break;
                    }
    
                    ty -= 1;
                }
            }

            let mut tx = x + 1;
            while tx > 0 && tx < plane.len() {
                if add_edge(&mut graph, node, plane, tx, y) {
                    break;
                }

                tx += 1;
            }

            if x > 0 {
                let mut tx = x - 1;
                while tx > 0 {
                    if add_edge(&mut graph, node, plane, tx, y) {
                        break;
                    }
    
                    tx -= 1;
                }
            }
        }
    }
    return graph;
}


fn add_edge(graph: &mut Graph, node: Node, plane: &Vec<Vec<u8>>, x: usize, y: usize) -> bool {
    if plane[y][x] != 1 {
        return true;
    }

    let ways = (plane[y-1][x] * 1) + (plane[y][x+1] * 2) + (plane[y+1][x] * 4) + (plane[y][x-1] * 8);

    if [5, 10].contains(&ways) {
        return false;
    }
    
    let next_node = Node::new(x as u16, y as u16);
    graph.add_node(next_node.clone());
    graph.add_edge((node, next_node));
    true
}
