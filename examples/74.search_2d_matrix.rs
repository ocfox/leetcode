// https://leetcode-cn.com/problems/search-a-2d-matrix/
fn main() {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        for x in matrix {
            match x.binary_search(&target) {
                Ok(x) => return true,
                Err(_) => continue,
            }
        }
        false
    }
}
