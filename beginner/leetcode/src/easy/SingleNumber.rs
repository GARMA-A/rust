#[allow(dead_code)]
struct Solution;

impl Solution{

    pub fn single_number(nums: Vec<i32>) -> i32 {
       let mut ans : i32 = nums[0];
       for &num in &nums[1..]{
              ans^=num;
       }
       ans
    }

}