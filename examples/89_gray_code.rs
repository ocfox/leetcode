pub fn gray_code(n: i32) -> Vec<i32> {
    let len = 2i32.pow(n as u32);
    let mut result = vec![0; len as usize];

    for i in 1..len {
        result[i as usize] = i ^ i >> 1;
        println!("{} {} {}", i, i >> 1, result[i as usize]);
    }
    result
}

fn main() {
    println!("{:?}", gray_code(4));
}
