// https://leetcode-cn.com/problems/repeated-string-match/
fn main() {
    pub fn repeated_string_match(a: String, b: String) -> i32 {
        for time in (b.len() / a.len())..=(b.len() / a.len() + 2) {
            if a.repeat(time).contains(&b) {
                return time as i32;
            }
        }
        -1
    }
    println!(
        "{}",
        repeated_string_match("abc".to_string(), "cabcabca".to_string())
    )
}
