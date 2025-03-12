pub fn dailyTemperatures(temps: Vec<i32>) -> Vec<i32> {
    let (mut ans, mut stack) = (vec![0; temps.len()], Vec::<usize>::new());
    for i in 0..temps.len() {
        while let Some(&prev_index) = stack.last() {
            if temps[i] > temps[prev_index] {
                ans[prev_index] = (i - prev_index) as i32;
                stack.pop();
            } else {
                break;
            }
        }
        stack.push(i);
    }

    ans
}
