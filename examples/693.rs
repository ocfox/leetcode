pub fn has_alternating_bits(n: i32) -> bool {
    let a = n ^ (n >> 1);
    a & (a + 1) == 0
}
fn main() {}

#[test]
fn test1() {
    assert_eq!(has_alternating_bits(5), true);
}
