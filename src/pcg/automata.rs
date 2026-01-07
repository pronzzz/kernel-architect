#![allow(dead_code)]
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeType {
    Void,
    Server,
    Database,
    Firewall,
    Gateway,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub node_type: NodeType,
    pub integrity: u8, // 0-100
    pub is_compromised: bool,
    pub description: String,
}

impl Node {
    pub fn new(node_type: NodeType) -> Self {
        Self {
            node_type,
            integrity: 100,
            is_compromised: false,
            description: String::new(),
        }
    }
}

pub struct Cluster {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Vec<Node>>,
}

impl Cluster {
    pub fn new(width: usize, height: usize) -> Self {
        let grid = vec![vec![Node::new(NodeType::Void); width]; height];
        Self { width, height, grid }
    }

    pub fn generate(&mut self) {
        let mut rng = rand::thread_rng();
        
        // Initialize random noise
        for y in 0..self.height {
            for x in 0..self.width {
                if rng.gen_bool(0.4) {
                    self.grid[y][x] = Node::new(NodeType::Server);
                }
            }
        }

        // Apply Cellular Automata rules
        // Rule: 4-5 rule (Alive if 4+ neighbors, born if 5+ neighbors) - simplified for demo
        for _ in 0..3 {
            let old_grid = self.grid.clone();
            for y in 1..self.height-1 {
                for x in 1..self.width-1 {
                    let neighbors = self.count_neighbors(&old_grid, x, y);
                    if self.grid[y][x].node_type != NodeType::Void {
                        if neighbors < 3 {
                            self.grid[y][x] = Node::new(NodeType::Void); // Die
                        }
                    } else {
                        if neighbors > 4 {
                            self.grid[y][x] = Node::new(NodeType::Server); // Born
                        }
                    }
                }
            }
        }

        // Post-processing: Assign specific roles (DB, Gateway)
        for y in 0..self.height {
            for x in 0..self.width {
                if self.grid[y][x].node_type == NodeType::Server {
                    let roll: f64 = rng.gen();
                    if roll < 0.1 {
                        self.grid[y][x].node_type = NodeType::Database;
                    } else if roll < 0.2 {
                        self.grid[y][x].node_type = NodeType::Firewall;
                    }
                }
            }
        }
    }

    fn count_neighbors(&self, grid: &Vec<Vec<Node>>, x: usize, y: usize) -> usize {
        let mut count = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 { continue; }
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if grid[ny as usize][nx as usize].node_type != NodeType::Void {
                    count += 1;
                }
            }
        }
        count
    }
}
