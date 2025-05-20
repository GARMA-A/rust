#![allow(non_snake_case, unused_variables)]

pub struct Solution;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let &mx = stones.iter().max().unwrap();
        let mut counting: Vec<i32> = vec![0; (mx + 1) as usize];
        for stone in stones {
            counting[stone as usize] += 1;
        }
        let mut current: usize = mx as usize;

        while current > 0 {
            if counting[current] % 2 == 1 {
                let mut next = current - 1;
                while next > 0 && counting[next] == 0 {
                    next -= 1;
                }

                if next == 0 {
                    return current as i32;
                }

                counting[current] -= 1;
                counting[next] -= 1;
                counting[current - next] += 1;
            } else {
                current -= 1;
            }
        }

        0
    }
}
