use std::collections::HashMap;

pub fn top_k_frequent(nums: Vec<i32>, mut k: i32) -> Vec<i32> {
       let mut ans:Vec<i32> = Vec::with_capacity(k as usize);
       let mut counter :HashMap<i32 , i32> = HashMap::new();
       for &num in &nums{
              *counter.entry(num).or_insert(0)+=1;
       }
       let mut size = nums.len();
       let mut bucket : Vec<Vec<i32>> = vec![vec![];size+1];
       for (&num , &freq) in &counter{
              bucket[freq as usize].push(num);
       }

       while k>0 {
              while bucket[size].is_empty(){
                     size-=1;
              }
              ans.push(bucket[size].pop().unwrap());
              k-=1;
       }

       ans
}