use std::io::{stdin, Read};

#[derive(Debug)]
struct Node {
    child_quantity: u32,
    metadata_quantity: u32,
    children: Vec<usize>,
    metadata: Vec<u32>,
}

impl Node {
    fn new(child_quantity: u32, metadata_quantity: u32) -> Node {
        let children = Vec::new();
        let metadata = Vec::new();
        Node {
            child_quantity,
            metadata_quantity,
            children,
            metadata,
        }
    }
}

struct Engine {
    numbers: Vec<u32>,
    numbers_index: usize,
    nodes: Vec<Node>,
}

impl Engine {
    fn new(numbers: Vec<u32>) -> Engine {
        let numbers_index = 0;
        let nodes = Vec::new();
        Engine {
            numbers,
            numbers_index,
            nodes,
        }
    }

    fn read_node(&mut self) {
        let child_quantity = self.numbers[self.numbers_index];
        self.numbers_index += 1;
        let metadata_quantity = self.numbers[self.numbers_index];
        self.numbers_index += 1;

        let node = Node::new(child_quantity, metadata_quantity);
        self.nodes.push(node);
        let own_nodes_index = self.nodes.len() - 1;

        for _child in 0..child_quantity {
            let child_nodes_index = self.nodes.len();
            self.nodes[own_nodes_index].children.push(child_nodes_index);
            self.read_node();
        }
        for _metadata in 0..metadata_quantity {
            let metadata = self.numbers[self.numbers_index];
            self.numbers_index += 1;
            self.nodes[own_nodes_index].metadata.push(metadata);
        }
    }

    fn metadata_sum(&self) -> u32 {
        let mut sum = 0;

        for node in self.nodes.iter() {
            for metadata in node.metadata.iter() {
                sum += metadata;
            }
        }

        sum
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let numbers: Vec<u32> = input
        .trim()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();

    let mut engine = Engine::new(numbers);
    engine.read_node();

    let part1 = engine.metadata_sum();
    println!("Part 1: the sum of all metadata entries is {}", part1);
}
