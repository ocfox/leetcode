fn main() {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut n = Vec::new();
        for i in 0..nums.len() {
            if nums[i] == target {
                n.push(i);
            }
        }
        if n.len() == 0 {
            return vec![-1, -1];
        }
        vec![n[0] as i32, n[n.len() - 1] as i32]
    }
}
