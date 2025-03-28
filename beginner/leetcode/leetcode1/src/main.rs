use std::collections::HashMap;

mod easy;
mod hard;
mod medium;

fn main() {
    let grid = vec![
        vec![0, 1, 1, 1, 0],
        vec![0, 1, 0, 1, 0],
        vec![1, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
    ];

    medium::numbetOfIslands::Solution::BFS(&grid);
}
