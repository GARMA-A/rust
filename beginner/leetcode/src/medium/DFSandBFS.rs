#![allow(dead_code)]

use std::collections::{HashMap, LinkedList, VecDeque};

pub struct Graph {
    size: i32,
    matrix: HashMap<i32, LinkedList<i32>>,
}
impl Graph {
    pub fn new() -> Self {
        Graph {
            size: 0,
            matrix: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, startNode: i32, endNode: i32) {
        let first_node = self.matrix.entry(startNode).or_insert(LinkedList::new());
        first_node.push_back(endNode);

        let second_node = self.matrix.entry(endNode).or_insert(LinkedList::new());
        second_node.push_back(startNode);
    }

    pub fn DFS(&mut self, startNode: i32) {
        let mut visited: HashMap<i32, bool> = HashMap::new();
        self.dfs_helper(&mut visited, &startNode);
    }
    pub fn dfs_helper(&self, visited: &mut HashMap<i32, bool>, startNode: &i32) {
        visited.insert(*startNode, true);
        print!("{} ", startNode);
        if let Some(neighbors) = self.matrix.get(startNode) {
            for &node in neighbors.iter() {
                if !*visited.entry(node).or_insert(false) {
                    visited.insert(node, true);
                    self.dfs_helper(visited, &node);
                }
            }
        }
    }
    pub fn BFS(&self, startNode: i32) {
        let mut queue: VecDeque<i32> = VecDeque::new();
        let mut visited: HashMap<i32, bool> = HashMap::new();
        visited.insert(startNode, true);
        queue.push_back(startNode);
        print!("{} ", startNode);
        while let Some(current) = queue.pop_front() {
            if let Some(neighbors) = self.matrix.get(&current) {
                for &neighbor in neighbors {
                    if !*visited.entry(neighbor).or_insert(false) {
                        print!("{} ", neighbor);
                        queue.push_back(neighbor);
                        visited.insert(neighbor, true);
                    }
                }
            }
        }
    }
}
