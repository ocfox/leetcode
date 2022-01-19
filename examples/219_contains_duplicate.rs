fn main() {
    let test = vec![1,2,3,1,2,3];
    println!("{}",contains_nearby_duplicate(test, 2));
}

pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    for i in 0..nums.len() {
        if map.contains_key(&nums[i]) {
            if (map.get(&nums[i]).unwrap() - i as i32).abs() <= k {
                return true;
            }
        }
        map.insert(nums[i], i as i32);
    }
    false
}
