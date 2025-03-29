struct Solution;

impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut max_area = 0;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    max_area = max_area.max(Self::dfs(&mut grid, i as i32, j as i32));
                }
            }
        }

        max_area
    }

    pub fn dfs(mut grid: &mut Vec<Vec<i32>>, i: i32, j: i32) -> i32 {
        if i < 0
            || i >= grid.len() as i32
            || j < 0
            || j >= grid[0].len() as i32
            || grid[i as usize][j as usize] == 0
        {
            return 0;
        }
        grid[i as usize][j as usize] = 0;
        let mut area = 1;
        area += Solution::dfs(&mut grid, i + 1, j);
        area += Solution::dfs(&mut grid, i - 1, j);
        area += Solution::dfs(&mut grid, i, j + 1);
        area += Solution::dfs(&mut grid, i, j - 1);
        area
    }
}
