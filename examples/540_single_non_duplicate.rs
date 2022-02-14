pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }
    let (mut left, mut right) = (0, 1);
    while right < nums.len() {
        if nums[left] != nums[right] {
            return nums[left];
        } else {
            left += 2;
            right += 2;
        }
    }
    nums[nums.len() - 1]
}
fn main() {}

#[cfg(test)]

mod tests {
    use crate::single_non_duplicate;

    #[test]
    fn duplicate_test() {
        let nums = vec![1, 1, 2, 3, 3, 4, 4, 8, 8];
        assert_eq!(single_non_duplicate(nums), 2);
    }

    #[test]
    fn duptest_2() {
        let nums = vec![3, 3, 7, 7, 10, 11, 11];
        assert_eq!(single_non_duplicate(nums), 10);
    }
}
