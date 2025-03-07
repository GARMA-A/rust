use std::cmp::{max, min};

pub fn trap(heights: Vec<i32>) -> i32 {
    let len = heights.len();
    let mut mx_vec_left: Vec<i32> = vec![0;len];
    let mut mx = heights[0];
    for (i , num) in heights.iter().enumerate() {
        mx = max(mx, *num);
        mx_vec_left[i] = mx;
    }

    mx = heights[len - 1];
    let mut mx_vec_right: Vec<i32> = vec![0;len];
    for i in (0..len).rev(){
        mx = max(mx, heights[i]);
        mx_vec_right[i] = mx;
    }
    let mut ans: i32 = 0;

    for (i, num) in heights.iter().enumerate() {
        let temp = min(mx_vec_left[i], mx_vec_right[i]) - num;
        if temp > 0 {
            ans += temp
        }
    }

    ans
}
