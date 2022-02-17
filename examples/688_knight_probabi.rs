pub fn knight_probability(n: i32, k: i32, row: i32, column: i32) -> f64 {
    let step = vec![1, 2, 1, -2, -1, 2, -1, -2, 1];
    let n = n as usize;
    let mut dp = vec![vec![1.0; n]; n];
    for _ in 0..k {
        let mut tmp = vec![vec![0.0; n]; n];
        for i in 0..n {
            for j in 0..n {
                for t in 0..8 {
                    let x = i as i32 + step[t];
                    let y = j as i32 + step[t + 1];
                    if x >= 0 && x < n as i32 && y >= 0 && y < n as i32 {
                        tmp[i][j] += dp[x as usize][y as usize] / 8.0;
                    }
                }
            }
        }
        dp = tmp;
    }
    dp[row as usize][column as usize]
}

fn main() {}
#[cfg(test)]

mod tests {
    use crate::knight_probability;

    #[test]
    fn probabi_test() {
        assert_eq!(knight_probability(3, 2, 0, 0), 0.0625);
    }

    #[test]
    fn probabi_2() {
        assert_eq!(knight_probability(1, 0, 0, 0), 1.0);
    }
}
