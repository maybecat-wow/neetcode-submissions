impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (idx, num) in nums.iter().enumerate() {
            for (idx2, num2) in nums.iter().enumerate() {
                if idx >= idx2 {
                    continue;
                }
                if *num + *num2 == target {
                    return Vec::from([idx as i32, idx2 as i32]);
                }
            }
        }
        panic!();
    }
}
