pub fn convert(s: String, num_rows: i32) -> String {
    let s = s.trim().chars().collect::<Vec<char>>();
    let mut res = vec![String::new(); (num_rows) as usize];

    let round = num_rows * 2 - 2;
    let mut col = 0;
    for i in 0..s.len() {
        let mut a = round - col;
        if col < num_rows {
            a = col;
        }
        res[a as usize].push(s[i]);

        if col < round - 1 {
            col += 1;
        } else {
            col = 0;
        }
    }
    let mut ans = String::new();
    for i in res {
        ans.push_str(&i)
    }
    ans
}

fn main() {}

#[cfg(test)]

mod test {
    use crate::convert;

    #[test]
    fn test1() {
        assert_eq!(
            convert("PAYPALISHIRING".to_string(), 4),
            "PINALSIGYAHRPI".to_string()
        )
    }

    #[test]
    fn test2() {
        assert_eq!(
            convert("PAYPALISHIRING".to_string(), 3),
            "PAHNAPLSIIGYIR".to_string()
        )
    }
}
