pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let mut res = Vec::new();
    for i in 0..matrix.len() {
        let mut min_idx = 0;
        let mut ok = true;
        for row in 1..matrix[0].len() {
            if matrix[i][row] < matrix[i][min_idx] {
                min_idx = row;
            }
        }
        for col in 0..matrix.len() {
            if matrix[col][min_idx] > matrix[i][min_idx] {
                ok = false;
                break;
            }
        }
        if ok {
            res.push(matrix[i][min_idx]);
        }
    }
    res
}

fn main() {}

#[cfg(test)]

mod tests {
    use crate::lucky_numbers;

    #[test]
    fn lucky_test() {
        let matrix = vec![vec![3, 7, 8], vec![9, 11, 13], vec![15, 16, 17]];
        assert_eq!(lucky_numbers(matrix), vec![15]);
    }
}
