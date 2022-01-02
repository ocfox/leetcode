fn main() {
    pub fn last_remaining(n: i32) -> i32 {
        if n == 1 {
            return 1;
        } else {
            return 2 * (n / 2 + 1 - last_remaining(n / 2));
        }
    }
    println!("{}", last_remaining(9))
}
