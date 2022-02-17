pub fn knight_probability(n: i32, k: i32, row: i32, column: i32) -> f64 {
    if k == 0 {
        return 1.0;
    }
    dfs(n, k, row, column) as f64 / 8.0f64.powi(k)
}

pub fn dfs(n: i32, k: i32, x: i32, y: i32) -> i32 {
    let step = vec![1, 2, 1, -2, -1, 2, -1, -2, 1];
    let mut res = 0;
    for i in 0..8 {
        let x = x + step[i];
        let y = y + step[i + 1];
        if x >= 0 && x < n as i32 && y >= 0 && y < n as i32 {
            if k > 1 {
                res += dfs(n, k - 1, x, y);
            } else {
                res += 1;
            }
        }
    }
    res
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
    fn probabi_0() {
        assert_eq!(knight_probability(3, 1, 0, 0), 0.25);
    }

    #[test]
    fn probabi_2() {
        assert_eq!(knight_probability(1, 0, 0, 0), 1.0);
    }

    #[test]
    fn probabi_3() {
        assert_eq!(knight_probability(3, 3, 0, 0), 0.015625);
    }
}
