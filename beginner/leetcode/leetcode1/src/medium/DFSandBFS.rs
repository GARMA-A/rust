#![allow(dead_code, unused_variables, non_snake_case)]
use std::collections::{HashMap, VecDeque};
pub struct Graph {
    pub adj_list: HashMap<char, Vec<char>>,
    pub size: i32,
}

impl Graph {
    pub fn new(size: i32) -> Self {
        Self {
            size,
            adj_list: HashMap::with_capacity(size as usize),
        }
    }

    pub fn add_edge(&mut self, u: char, v: char, undirect: bool) {
        self.adj_list
            .entry(u)
            .and_modify(|vec| vec.push(v))
            .or_insert(vec![v]);

        if undirect {
            self.adj_list
                .entry(v)
                .and_modify(|vec| vec.push(u))
                .or_insert(vec![u]);
        }
    }

    pub fn add_vertex(&mut self, u: char) {
        self.adj_list.entry(u).or_insert(vec![]);
    }

    pub fn iterative_BFS(&self, start: char) {
        let mut qu: VecDeque<char> = VecDeque::new();
        let mut visited: HashMap<char, bool> = HashMap::new();

        qu.push_back(start);

        visited.insert(start, true);

        while let Some(node) = qu.pop_front() {
            print!("{} ", node);

            if let Some(neighbors) = self.adj_list.get(&node) {
                for &neighbor in neighbors {
                    if !visited.get(&neighbor).unwrap_or(&false) {
                        qu.push_back(neighbor);
                        visited.insert(neighbor, true);
                    }
                }
            }
        }
    }
    // this version i complet it on my own

    pub fn bfs(&self, start: char) {
        let mut dq: VecDeque<char> = VecDeque::new();
        let mut visited: HashMap<char, bool> = HashMap::new();
        dq.push_back(start);
        visited.insert(start, true);

        while let Some(top) = dq.pop_front() {
            print!("{} ", top);
            if let Some(nodes) = self.adj_list.get(&top) {
                for &node in nodes {
                    if !*visited.entry(node).or_insert(false) {
                        dq.push_back(node);
                        visited.insert(node, true);
                    }
                }
            }
        }
    }

    pub fn dfs_with_help(&self, curNode: char, visited: &mut HashMap<char, bool>) {
        if let Some(nodes) = self.adj_list.get(&curNode) {
            print!("{} ", curNode);
            visited.insert(curNode, true);
            for &node in nodes {
                if !*visited.entry(node).or_insert(false) {
                    self.dfs_with_help(node, visited);
                }
            }
        }
    }

    pub fn dfs(&self, curNode: char) {
        let mut visited: HashMap<char, bool> = HashMap::new();
        self.dfs_helper(curNode, &mut visited);
    }

    pub fn dfs_helper(&self, curNode: char, visited: &mut HashMap<char, bool>) {
        if let Some(nodes) = self.adj_list.get(&curNode) {
            visited.insert(curNode, true);
            print!("{} ", curNode);
            for &node in nodes {
                if !*visited.entry(node).or_insert(false) {
                    self.dfs_helper(node, visited);
                }
            }
        }
    }
}
