pub fn optimal_division(nums: Vec<i32>) -> String {
    let len = nums.len();
    if len == 1 {
        return format!("{}", nums[0]);
    }
    if len == 2 {
        return format!("{}/{}", nums[0], nums[1]);
    }
    let mut res = format!("{}/({}", nums[0], nums[1]);
    for i in 2..len {
        res.push_str(&format!("/{}", nums[i]))
    }
    res.push(')');
    res
}

fn main() {}

#[cfg(test)]

mod test {
    use crate::optimal_division;

    #[test]
    fn test1() {
        let test = vec![1000, 100, 10, 2];
        assert_eq!(optimal_division(test), "1000/(100/10/2)".to_string())
    }
}
