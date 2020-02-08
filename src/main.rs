use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Union {
    from: u8,
    to: u8,
}

#[derive(Serialize, Deserialize)]
struct AppConfig {
    size: u8,
    unions: Vec<Union>,
}

struct Calculator {
    state: Vec<u8>,
}

impl Calculator {
    fn new(size: u8) -> Calculator {
        let mut result = Calculator {
            state: vec![0; size as usize],
        };

        for i in 0..size {
            result.state[i as usize] = i;
        }

        return result;
    }

    fn find_root(&mut self, index: u8) -> u8 {
        let mut i = index;
        while i != self.state[i as usize] {
            i = self.state[i as usize];
        }

        return i;
    }

    fn union(&mut self, node_a: u8, node_b: u8) -> () {
        let root_a = self.find_root(node_a);
        let root_b = self.find_root(node_b);

        self.state[root_a as usize] = root_b;
    }

    fn is_connected(&mut self, node_a: u8, node_b: u8) -> bool {
        return self.find_root(node_a) == self.find_root(node_b);
    }
}

fn main() {
    println!("Quick Union Example");

    let raw_config = fs::read_to_string("input.json").unwrap();
    let config: AppConfig = serde_json::from_str(&raw_config).unwrap();

    let mut calculator = Calculator::new(config.size);
    for (_i, pair) in config.unions.iter().enumerate() {
        calculator.union(pair.from, pair.to);
    }

    let connected = calculator.is_connected(2, 13);
    println!("2 and 13 connected: {}", connected);
}
