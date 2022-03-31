fn main() {}

pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
    (left..=right)
        .filter(|&n| {
            let mut x = n;
            while x > 0 {
                let digit = x % 10;
                if digit == 0 || n % digit != 0 {
                    return false;
                }
                x /= 10;
            }
            true
        })
        .collect()
}

#[test]
fn test1() {
    assert_eq!(self_dividing_numbers(47, 85), vec![48, 55, 66, 77]);
}
