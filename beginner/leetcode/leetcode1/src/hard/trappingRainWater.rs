use std::cmp::{max, min};

pub fn trap(heights: Vec<i32>) -> i32 {
    let mut mx_vec_left: Vec<i32> = Vec::new();
    let mut mx = heights[0];
    for num in &heights {
        mx = max(mx, *num);
        mx_vec_left.push(mx);
    }

    mx = heights[heights.len() - 1];
    let mut mx_vec_right: Vec<i32> = Vec::new();
    for num in heights.iter().rev() {
        mx = max(mx, *num);
        mx_vec_right.push(mx);
    }
    mx_vec_right.reverse();

    let mut ans: i32 = 0;

    for (i, num) in heights.iter().enumerate() {
        let temp = min(mx_vec_left[i], mx_vec_right[i]) - num;
        if temp > 0 {
            ans += temp
        }
    }

    ans
}
