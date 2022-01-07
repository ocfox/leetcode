pub fn max_depth(s: String) -> i32 {
    let (mut res, mut now) = (0, 0);
    for char in s.chars() {
        if char == '(' {
            now += 1;
            res = res.max(now);
        } else if char == ')' {
            now -= 1;
            res = res.max(now);
        }
    }
    if res > 0 {
        res
    } else {
        0
    }
}
fn main() {
    let test = "(4)+(5)-(((3)))".to_string();
    println!("{}", max_depth(test));
}
