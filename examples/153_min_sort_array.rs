// https://leetcode-cn.com/problems/find-minimum-in-rotated-sorted-array/
fn main() {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        nums[0]
    }
    let test = vec![1,2,4,56,2,-4];
    println!("{}",find_min(test));
}
