pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
    let mut idx = 0;
    while idx < bits.len() {
        if idx == bits.len() - 1 {
            return true;
        } else {
            if bits[idx] == 0 {
                idx += 1;
            } else {
                idx += 2;
            }
        }
    }
    false
}

fn main() {}

#[cfg(test)]

mod tests {
    use crate::is_one_bit_character;

    #[test]
    fn bit_test() {
        let bits = vec![1, 1, 1, 0];
        assert_eq!(is_one_bit_character(bits), false);
    }
}
