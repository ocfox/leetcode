// https://leetcode-cn.com/problems/convert-1d-array-into-2d-array/
fn main() {
    pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        let (m, n) = (m as usize, n as usize);
        if original.len() != m * n {
            return vec![];
        }
        let mut res = vec![vec![0; n]; m];
        for i in 0..m {
            for x in 0..n {
                res[i][x] = original[i * n + x];
            }
        }
        res
    }
}
