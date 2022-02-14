fn main() {}
pub fn max_number_of_balloons(text: String) -> i32 {
    let mut res = vec![0; 5];
    for i in text.chars() {
        match i {
            'b' => res[0] += 1,
            'a' => res[1] += 1,
            'l' => res[2] += 1,
            'o' => res[3] += 1,
            'n' => res[4] += 1,
            _ => continue,
        }
    }
    res[2] /= 2;
    res[3] /= 2;
    res.sort_unstable();
    res[0]
}

#[cfg(test)]

mod tests {
    use crate::max_number_of_balloons;

    #[test]
    fn balloon_test() {
        let text = "loonbalxballpoon".to_string();
        assert_eq!(max_number_of_balloons(text), 2);
    }
}
