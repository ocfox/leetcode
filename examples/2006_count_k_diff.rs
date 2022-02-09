pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
    let mut res = 0;
    for i in 0..nums.len() {
        for j in i..nums.len() {
            if (nums[i] - nums[j]).abs() == k {
                res += 1;
            }
        }
    }
    res
}

fn main() {}

#[cfg(test)]

mod tests {
    use crate::count_k_difference;

    #[test]
    fn count_k_test() {
        let t = vec![3, 2, 1, 5, 4];
        assert_eq!(count_k_difference(t, 2), 3);
    }
}
