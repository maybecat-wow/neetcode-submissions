impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut k = nums.len();
        let mut idx = 0;
        while idx < k {
            if *nums.get(idx).unwrap() == val {
                nums.swap(idx, k - 1);
                k -= 1;
            } else {
                idx += 1;
            }
        }
        k as i32
    }
}
