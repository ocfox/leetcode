fn main() {}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn simplified_fractions(n: i32) -> Vec<String> {
    let mut ans = vec![];
    for y in 2..=n {
        for x in 1..y {
            if gcd(x, y) == 1 {
                ans.push(format!("{}/{}", x, y));
            }
        }
    }
    ans
}

#[cfg(test)]

mod tests {
    use crate::simplified_fractions;

    #[test]
    fn simplified_test() {
        let res = vec!["1/2".to_string(), "1/3".to_string(), "2/3".to_string()];
        assert_eq!(simplified_fractions(3), res);
    }
}
