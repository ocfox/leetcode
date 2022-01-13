pub fn increasing_triplet(nums: Vec<i32>) -> bool {
    let (mut fir, mut sec) = (i32::MAX, i32::MAX);
    for i in 0..nums.len() {
        if nums[i] <= fir {
            fir = nums[i];
        } else if nums[i] <= sec {
            sec = nums[i];
        } else {
            return true;
        }
    }
    return false;
}

fn main() {
    let test = vec![20, 100, 10, 12, 5, 13];
    println!("{}", increasing_triplet(test));
}
