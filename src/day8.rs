use std::fs;
// use itertools::Itertools;

#[derive(Debug)]
struct Node {
    x: u32,
    y: u32,
    z: u32,
}

struct NodeGraph {
    nodes: Vec<Node>,
    parent: Vec<usize>,
    sizes: Vec<usize>,
}


impl NodeGraph {
    fn new() -> Self {
        NodeGraph {
            nodes: Vec::new(),
            parent: Vec::new(),
            sizes: Vec::new(),
        }
    }

    fn add_node(&mut self, coords: Vec<u32>) {
        let index = self.nodes.len();

        self.nodes.push(Node {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        });
        self.parent.push(index);
        self.sizes.push(1 as usize)
    }

    fn get_parent(&mut self, index: usize) -> usize {
        // Path Compression
        if self.parent[index] != index {
            self.parent[index] = self.get_parent(self.parent[index])
        }
        self.parent[index]
    }

    fn distance(&self, i: usize, j: usize) -> f64 {
        let dx = (self.nodes[i].x as f64 - self.nodes[j].x as f64).powi(2);
        let dy = (self.nodes[i].y as f64 - self.nodes[j].y as f64).powi(2);
        let dz = (self.nodes[i].z as f64 - self.nodes[j].z as f64).powi(2);
        (dx + dy + dz).sqrt()
    }
}

pub fn solve() {
    // Read and Construct Input Graph
    let input = fs::read_to_string("src/input/day8.txt").unwrap();
    let mut graph = NodeGraph::new();
    for line in input.lines() {
        let coords: Vec<u32> = line.split(',')
                                .map(|s| s.parse::<u32>().unwrap())
                                .collect();
        graph.add_node(coords);
    }

    let len: usize = graph.nodes.len();
    let mut distances: Vec<(usize, usize, f64)> = Vec::new();

    for i in 0..len {
        for j in (i+1)..len {
            distances.push((i, j, graph.distance(i, j)));
        }
    }
    distances.sort_by(|(_, _, a), (_, _, b)| a.partial_cmp(b).unwrap());

    // let mut count = 0;
    // let mut result = 1;
    let mut last_connection: Option<(usize, usize)> = None;
    for &(i, j, _) in &distances {

        // Part 1
        // if count >= 1000 {
        //     result = graph.sizes
        //         .iter()
        //         .sorted()
        //         .rev()
        //         .take(3)
        //         .product::<usize>();
        //     break;
        // }

        let parent_i = graph.get_parent(i);
        let parent_j = graph.get_parent(j);

        if parent_i != parent_j {

            // Attaching smaller to bigger
            if graph.sizes[parent_i] >= graph.sizes[parent_j] {
                graph.parent[parent_j] = parent_i;
                graph.sizes[parent_i] += graph.sizes[parent_j];
                graph.sizes[parent_j] = 1;
            } else {
                graph.parent[parent_i] = parent_j;
                graph.sizes[parent_j] += graph.sizes[parent_i];
                graph.sizes[parent_i] = 1;
            }
        }

        let mut all_connected = true;
        let first_root = graph.get_parent(0);
        for k in 1..len {
            if graph.get_parent(k) != first_root {
                all_connected = false;
                break;
            }
        }
        if all_connected {
            last_connection = Some((i, j));
            break;
        }
    }
    if let Some((last_i, last_j)) = last_connection {
        let part2_result = graph.nodes[last_i].x as usize * graph.nodes[last_j].x as usize;
        println!("Part 2: {} ({} * {})", 
                 part2_result, graph.nodes[last_i].x, graph.nodes[last_j].x);
    }
}
