pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
    let n_sum = mean * (n + rolls.len() as i32) - rolls.iter().sum::<i32>();

    if n_sum < n || n_sum > n * 6 {
        return vec![];
    }

    let mut result = vec![n_sum / n; n as usize];
    for i in 0..n_sum % n {
        result[i as usize] += 1;
    }

    result
}
fn main() {}

#[cfg(test)]

mod test {
    use crate::missing_rolls;

    #[test]
    fn test1() {
        let rolls = vec![3, 2, 4, 3];
        assert_eq!(vec![6, 6], missing_rolls(rolls, 4, 2));
    }
}
