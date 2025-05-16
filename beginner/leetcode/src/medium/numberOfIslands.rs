#![allow(dead_code, unused_variables, non_snake_case)]

use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn bfs(&self, grid: &mut Vec<Vec<char>>, r: i32, c: i32) -> () {
        let mut qu: VecDeque<(i32, i32)> = VecDeque::new();
        let Directions: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        qu.push_back((r, c));
        grid[r as usize][c as usize] = '0';
        while let Some(cur_node) = qu.pop_front() {
            for &(dx, dy) in &Directions {
                let x = cur_node.0 + dx;
                let y = cur_node.1 + dy;
                if x >= 0
                    && y >= 0
                    && (x as usize) < grid.len()
                    && (y as usize) < grid[0].len()
                    && grid[x as usize][y as usize] == '1'
                {
                    grid[x as usize][y as usize] = '0';
                    qu.push_back((x, y));
                }
            }
        }
    }

    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut islands: i32 = 0;
        let ROW = grid.len();
        let COL = grid[0].len();

        for r in 0..ROW {
            for c in 0..COL {
                if grid[r][c] == '1' {
                    Self.bfs(&mut grid, r as i32, c as i32);
                    islands += 1;
                }
            }
        }

        islands
    }
}
