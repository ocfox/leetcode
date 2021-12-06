fn main() {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        match nums.iter().position(|&x| x == target) {
            Some(i) => return i as i32,
            None => return -1,
        }
    }
}
