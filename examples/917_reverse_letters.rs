pub fn reverse_only_letters(s: String) -> String {
    let mut s = s.into_bytes();
    let mut cs = s
        .iter()
        .filter(|c| c.is_ascii_alphabetic())
        .cloned()
        .collect::<Vec<u8>>();
    for c in s.iter_mut() {
        if c.is_ascii_alphabetic() {
            *c = cs.pop().unwrap();
        }
    }
    String::from_utf8_lossy(&s).to_string()
}

fn main() {
    let s = "ask-djfhdfkla-jsdf-a".to_string();
    println!("{}", reverse_only_letters(s));
}
