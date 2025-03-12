#[allow(dead_code)]
struct Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let len1 = s1.len();
        let len2 = s2.len();

        if len1 > len2 {
            return false;
        }

        let mut s1_freq = [0; 26];
        let mut window_freq = [0; 26];

        for ch in s1.chars() {
            s1_freq[(ch as u8 - b'a') as usize] += 1;
        }

        for i in 0..len2 {
            window_freq[(s2.as_bytes()[i] - b'a') as usize] += 1;

            if i >= len1 {
                window_freq[(s2.as_bytes()[i - len1] - b'a') as usize] -= 1;
            }

            if window_freq == s1_freq {
                return true;
            }
        }

        false
    }
}