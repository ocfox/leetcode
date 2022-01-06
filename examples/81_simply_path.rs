pub fn simplify_path(path: String) -> String {
    let mut res = Vec::new();
    let dot = "..";
    for pos in path.split('/') {
        if !dot.contains(pos) {
            res.push(pos.to_string());
        } else if pos == dot {
            res.pop();
        }
    }
    if res.len() == 0 {
        return "/".to_string();
    }
    res.iter().map(|x| "/".to_string() + x).collect()
}
fn main() {
    let ans = simplify_path("/th/ii/../i".to_string());
    println!("{}", ans);
}
