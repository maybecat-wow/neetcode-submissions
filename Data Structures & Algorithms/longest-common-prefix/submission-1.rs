impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
               let mut ans = String::new();
        ans = strs.first().unwrap().clone();
        for item in strs {
            let mut max_length = item.len();
            let ans_char = ans.chars();
            for (idx, (letter1, letter2)) in item.chars().zip(ans_char).enumerate() {
                if letter1 != letter2 {
                    max_length = idx;
                    break;
                }
            }
            ans.truncate(max_length);
        }
        ans
    }
}
