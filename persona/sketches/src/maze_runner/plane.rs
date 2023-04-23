use crate::maths::graph::{Graph, Node};


pub fn generate_graph(plane: &Vec<Vec<u8>>) -> Graph {
    let mut graph = Graph::new();
    for (i, row) in plane.iter().enumerate() {
        for (j, b) in row.iter().enumerate() {

            if *b != 1 {
                continue;
            }

            if !node_condition(i, j, plane) {
                continue;
            }

            let node = Node::new(i as i32, j as i32);
            if graph.add_node(node.clone()) {
                continue;
            }

            let mut ti = i + 1;
            while ti < plane.len() {
                if add_edge(&mut graph, &node, plane, ti, j) {
                    break;
                }

                ti += 1;
            }

            if i > 0 {
                let mut ti = i - 1;
                while ti > 0 {
                    if add_edge(&mut graph, &node, plane, ti, j) {
                        break;
                    }
    
                    ti -= 1;
                }
            }

            let mut tj = j + 1;
            while tj > 0 && tj < plane.len() {
                if add_edge(&mut graph, &node, plane, i, tj) {
                    break;
                }

                tj += 1;
            }

            if j > 0 {
                let mut tj = j - 1;
                while tj > 0 {
                    if add_edge(&mut graph, &node, plane, i, tj) {
                        break;
                    }
    
                    tj -= 1;
                }
            }
        }
    }
    return graph;
}



fn add_edge(graph: &mut Graph, node: &Node, plane: &Vec<Vec<u8>>, i: usize, j: usize) -> bool {
    if plane[i][j] != 1 {
        return true;
    }

    if !node_condition(i, j, plane) {
        return false;
    }
    
    let next_node = Node::new(i as i32, j as i32);
    graph.add_node(next_node.clone());
    graph.add_edge((node.id(), next_node.id()));
    true
}


fn node_condition(i: usize, j: usize, plane: &Vec<Vec<u8>>) -> bool {
    i == 0 || 
    i == plane.len() - 1 || 
    j == 0 || 
    j == plane[0].len() - 1 || 
    plane[i-1][j] != plane[i+1][j] || 
    plane[i][j-1] != plane[i][j+1] ||
    plane[i-1][j] == plane[i][j-1] 
}