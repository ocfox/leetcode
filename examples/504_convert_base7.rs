pub fn convert_to_base7(num: i32) -> String {
    if num == 0 {
        return "0".to_string();
    }
    let mut ans = "".to_string();
    let mut n = num.abs();
    while n != 0 {
        ans += &(n % 7).to_string();
        n /= 7;
    }
    if num.signum() < 0 {
        ans += "-";
    }
    ans.chars().rev().collect::<String>()
}
fn main() {}

#[cfg(test)]

mod test {
    use crate::convert_to_base7;

    #[test]
    fn test1() {
        assert_eq!(convert_to_base7(100), "202".to_string());
    }
}
