fn main() {}

pub fn plates_between_candles(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let s = s.into_bytes();
    let mut a = Vec::with_capacity(s.len());
    for (i, c) in s.into_iter().enumerate() {
        if c == b'|' {
            a.push(i as i32);
        }
    }
    let mut ans = Vec::with_capacity(queries.len());
    for q in queries.into_iter() {
        let i = match a.binary_search(&q[0]) {
            Ok(t) => t,
            Err(t) => t,
        };
        let j = match a.binary_search(&q[1]) {
            Ok(t) => t,
            Err(t) => {
                if t > 0 {
                    t - 1
                } else {
                    0
                }
            }
        };
        if j > i {
            ans.push(a[j] - a[i] - (j - i) as i32);
        } else {
            ans.push(0);
        }
    }
    ans
}

#[cfg(test)]

mod test {
    use crate::plates_between_candles;

    #[test]
    fn test1() {
        let s = "***|**|*****|**||**|*".to_string();
        let que = vec![vec![1, 17], vec![4, 5]];
        let res = vec![9, 0];
        assert_eq!(plates_between_candles(s, que), res);
    }
}
