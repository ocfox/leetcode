// https://leetcode-cn.com/problems/valid-tic-tac-toe-state/
fn main() {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        let mut bottles = num_bottles;
        let mut res = num_bottles;
        while bottles >= num_exchange {
            res += bottles / num_exchange;
            bottles = bottles % num_exchange + (bottles / num_exchange);
        }
        res
    }
    println!("{}", num_water_bottles(6, 2));
}
