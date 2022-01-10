fn main() {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let (mut big, mut rely) = (0, 0);
        for num in nums {
            let tmp;
            tmp = big;
            big = big.max(rely + num);
            rely = tmp;
        }
        big
    }
    println!("{}", rob(vec![1, 2, 3, 1]))
}
