use std::collections::HashMap;
pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
    let mut map = HashMap::new();
    let mut res = Vec::new();
    for i in s1.split_whitespace() {
        map.insert(i, !map.contains_key(i));
    }
    for i in s2.split_whitespace() {
        map.insert(i, !map.contains_key(i));
    }
    for (k, v) in map {
        if v {
            res.push(k.to_string());
        }
    }
    res
}

fn main() {}

#[cfg(test)]

mod tests {
    use crate::uncommon_from_sentences;

    #[test]
    fn uncommon_test() {
        let s1 = "this apple is sweet".to_string();
        let s2 = "this apple is sour".to_string();
        assert_eq!(
            uncommon_from_sentences(s1, s2),
            vec!["sour".to_string(), "sweet".to_string()]
        );
    }
}
