// https://leetcode-cn.com/problems/find-the-town-judge/
fn main() {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut status = vec![0; n as usize];
        for man in trust {
            status[man[0] as usize - 1] -= 1;
            status[man[1] as usize - 1] += 1;
        }

        for (judge, trust) in status.iter().enumerate() {
            if *trust == n - 1 {
                return judge as i32 + 1;
            }
        }
        -1
    }
    let test = vec![vec![1, 2]];
    println!("{}", find_judge(2, test));
}
