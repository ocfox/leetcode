fn main() {
    pub fn truncate_sentence(s: String, k: i32) -> String {
        let mut count = 0;
        let mut res = String::new();
        for char in s.chars() {
            if char == ' ' {
                count += 1;
            }
            res.push(char);
            if count == k {
                break;
            }
        }
        res.trim().to_string()
    }
    let test = "this is a test Really?".to_string();
    println!("{}", truncate_sentence(test, 4));
}
