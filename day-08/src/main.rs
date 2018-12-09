use std::io::{stdin, Read};

#[derive(Debug)]
struct Node {
    index: usize,
    child_quantity: u32,
    metadata_quantity: u32,
    children: Vec<usize>,
    metadata: Vec<u32>,
}

impl Node {
    fn new(index: usize, child_quantity: u32, metadata_quantity: u32) -> Node {
        let children = Vec::new();
        let metadata = Vec::new();
        Node {
            index,
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
    next_node_index: usize,
}

impl Engine {
    fn new(numbers: Vec<u32>) -> Engine {
        let numbers_index = 0;
        let nodes = Vec::new();
        let next_node_index = 0;
        Engine {
            numbers,
            numbers_index,
            nodes,
            next_node_index,
        }
    }

    fn read_node(&mut self) {
        let child_quantity = self.numbers[self.numbers_index];
        self.numbers_index += 1;
        let metadata_quantity = self.numbers[self.numbers_index];
        self.numbers_index += 1;

        let own_node_index = self.next_node_index;
        self.next_node_index += 1;
        let node = Node::new(own_node_index, child_quantity, metadata_quantity);
        self.nodes.push(node);

        for _child in 0..child_quantity {
            let child_node_index = self.next_node_index;
            self.nodes[own_node_index].children.push(child_node_index);
            self.read_node();
        }
        for _metadata in 0..metadata_quantity {
            let metadata = self.numbers[self.numbers_index];
            self.numbers_index += 1;
            self.nodes[own_node_index].metadata.push(metadata);
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

    fn root_node_value(&mut self) -> u32 {
        let mut values = vec![0; self.nodes.len()];

        for node in self.nodes.iter().rev() {
            if node.child_quantity == 0 {
                values[node.index] = node.metadata.iter().sum();
            } else {
                let metadata: Vec<usize> = node
                    .metadata
                    .iter()
                    .filter(|&&x| x > 0 && x as usize <= node.children.len())
                    .map(|&x| x as usize - 1)
                    .collect();
                for metadatum in metadata {
                    let child_index: usize = node.children[metadatum];
                    values[node.index] += values[child_index];
                }
            }
        }

        values[0]
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

    let part2 = engine.root_node_value();
    println!("Part 2: the value of the root node is {}", part2);
}
