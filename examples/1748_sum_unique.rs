pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }
    let mut nums = nums;
    let mut res = 0;
    nums.sort_unstable();
    for i in 1..(nums.len() - 1) {
        if nums[i] != nums[i - 1] && nums[i] != nums[i + 1] {
            res += nums[i];
        }
    }
    if nums[0] != nums[1] {
        res += nums[0];
    }
    if nums[nums.len() - 1] != nums[nums.len() - 2] {
        res += nums[nums.len() - 1];
    }
    res
}

fn main() {}
#[cfg(test)]

mod tests {
    use crate::sum_of_unique;

    #[test]
    fn sum_test() {
        let test = vec![1, 3, 4, 56, 6, 3, 56];
        assert_eq!(sum_of_unique(test), 3);
    }
}
