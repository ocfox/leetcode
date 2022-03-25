pub fn nearest_palindromic(n: String) -> String {
    let mut n = n.chars().collect::<Vec<char>>();
    let len = n.len();
    for i in 0..len / 2 {
        if n[i] != n[len - 1 - i] {
            n[len - 1 - i] = n[i];
        }
    }
    n.iter().collect()
}
fn main() {
    println!("{}", nearest_palindromic("123".to_string()))
}
