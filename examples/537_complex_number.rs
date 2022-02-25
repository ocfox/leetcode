pub fn complex_number_multiply(num1: String, num2: String) -> String {
    let num1 = comparse(num1);
    let num2 = comparse(num2);
    format!(
        "{}+{}i",
        num1.0 * num2.0 - num1.1 * num2.1,
        num1.0 * num2.1 + num1.1 * num2.0
    )
}

pub fn comparse(s: String) -> (i32, i32) {
    let idx = s.find('+').unwrap();
    (
        (s[0..idx].parse().unwrap()),
        (s[idx + 1..s.len() - 1].parse().unwrap()),
    )
}

fn main() {}

#[cfg(test)]

mod test {
    use crate::complex_number_multiply;

    #[test]
    fn test1() {
        let num1 = "1+1i".to_string();
        let num2 = "1+1i".to_string();
        assert_eq!(complex_number_multiply(num1, num2), "0+2i".to_string())
    }
}
