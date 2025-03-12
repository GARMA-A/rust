#[allow(dead_code)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn encode(strs: Vec<String>) -> String {
        let mut ans: String = String::new();
        for str in strs {
            ans += &(str.len().to_string() + ":" + &str);
        }
        ans
    }

    #[allow(dead_code)]
    pub fn decode(str: String) -> Vec<String> {
        let mut ans = Vec::new();
        let mut i = 0;
        while i < str.len() {
            let end_of_integer = str[i..].find(':').unwrap() + i;

            let size_of_the_str: usize = str[i..end_of_integer].parse().unwrap();

            i = end_of_integer + 1;

            let temp_str = str[i..i + size_of_the_str].to_string();

            i += size_of_the_str;

            ans.push(temp_str);
        }
        ans
    }
}
