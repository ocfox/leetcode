pub fn add_digits(num: i32) -> i32 {
    (num - 1) % 9 + 1
}

fn main() {}
#[cfg(test)]

mod test {
    use crate::add_digits;

    #[test]
    fn test1() {
        assert_eq!(add_digits(38), 2);
    }
}

