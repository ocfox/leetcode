fn main() {
    pub fn climb_stairs(n: i32) -> i32 {
        let sqrt5 = 5.0_f64.sqrt();
        let fib = ((1.0 + sqrt5) / 2.0).powi(n + 1) - ((1.0 - sqrt5) / 2.0).powi(n + 1);
        ((fib / sqrt5) + 0.1) as i32
    }
    println!("{}", climb_stairs(13))
}
