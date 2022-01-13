pub fn dominant_index(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return 0;
    }
    let (mut max, mut index, mut sec) = (0, 0, 0);
    for (i, dex) in nums.iter().zip(0..nums.len()) {
        if i > &max {
            sec = max;
            max = *i;
            index = dex;
        } else if i > &sec {
            sec = *i;
        }
    }
    if max >= sec * 2 {
        return index as i32;
    }
    -1
}

fn main() {
    let test = vec![0, 4, 5, 2, 6, 9, 34];
    println!("{}", dominant_index(test))
}
