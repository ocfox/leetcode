pub fn number_of_steps(num: i32) -> i32 {
    let mut times = 0;
    let mut num = num;
    while num != 0 {
        if num % 2 == 0 {
            num /= 2;
            times += 1;
        } else {
            num -= 1;
            times += 1;
        }
    }
    times
}

fn main() {}

#[cfg(test)]

mod tests {
    use crate::number_of_steps;

    #[test]
    fn number_test() {
        assert_eq!(number_of_steps(14), 6);
    }
}
