fn main() {}
pub fn cal_points(ops: Vec<String>) -> i32 {
    let mut score: Vec<i32> = Vec::new();
    for op in ops.iter() {
        match op as &str {
            "C" => {
                score.pop();
            }
            "D" => score.push(score[score.len() - 1] * 2),
            "+" => score.push(score[score.len() - 1] + score[score.len() - 2]),
            _ => score.push(op.parse::<i32>().unwrap()),
        }
    }
    score.iter().sum()
}

#[cfg(test)]

mod test {
    use crate::cal_points;

    #[test]
    fn test1() {
        let t = ["5", "2", "C", "D", "+"];
        let mut ops = Vec::new();
        for i in t.iter() {
            ops.push(i.to_string());
        }
        assert_eq!(30, cal_points(ops))
    }
}
