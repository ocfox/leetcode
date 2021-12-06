use std::cmp;

// https://leetcode-cn.com/problems/find-minimum-in-rotated-sorted-array/
fn main() {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let (mut dp, mut res) = (nums[0], nums[0]);
        for i in 1..nums.len() {
            dp = std::cmp::max(0, dp) + nums[i];
            res = std::cmp::max(dp, res);
        }
        res
    }
}
