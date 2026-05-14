impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
            let ans = nums.clone();
            [nums, ans].concat()
    }
}
