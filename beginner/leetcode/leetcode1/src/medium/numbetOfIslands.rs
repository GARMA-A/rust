#![allow(dead_code, unused_variables, non_snake_case)]

use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn BFS(grid: &Vec<Vec<i32>>) -> i32 {
        const Directions: [(i8, i8); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        let mut island_num: i32 = 0;
        let mut prev_level_contain_one: bool = false;
        let rows = grid.len();
        let cols = grid[0].len();
        let mut dq: VecDeque<(usize, usize)> = VecDeque::new();
        let mut visited: Vec<Vec<bool>> = vec![vec![false; cols]; rows];
        dq.push_back((0, 0));
        visited[0][0] = true;
        while let Some((x, y)) = dq.pop_front() {
            for &(dx, dy) in Directions.iter() {
                let nx = x.wrapping_add(dx as usize);
                let ny = y.wrapping_add(dy as usize);
                if nx < rows && ny < cols && !visited[nx][ny] {
                    visited[nx][ny] = true;
                    dq.push_back((nx, ny));
                }
            }
        }
        island_num
    }
}
