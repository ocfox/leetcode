pub fn reverse_prefix(word: String, ch: char) -> String {
    match word.find(ch) {
        Some(a) => {
            let mut res: String = word[0..=a].chars().rev().collect();
            res.push_str(&word[a+1..]);
            res
        }
        _ => return word,
    }
}

fn main() {}

#[cfg(test)]

mod tests {
    use crate::reverse_prefix;

    #[test]
    fn rev_test() {
        assert_eq!(
            reverse_prefix("faerd".to_string(), 'e'),
            "eafrd".to_string()
        );
    }
}
