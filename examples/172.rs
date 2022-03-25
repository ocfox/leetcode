pub fn trailing_zeroes(n: i32) -> i32 {
    let mut result = 0;
    let mut current = 5;
    while current <= n {
        result += n / current;
        current *= 5;
    }
    result
}

fn main() {
    trailing_zeroes(30);
}

#[cfg(test)]

mod test {
    use crate::trailing_zeroes;

    #[test]
    fn test1() {
        assert_eq!(trailing_zeroes(10), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(trailing_zeroes(3), 0);
    }
}
