pub fn count_good_rectangles(rectangles: Vec<Vec<i32>>) -> i32 {
    let mut rect = Vec::new();
    let mut res = 1;
    for i in rectangles {
        rect.push(i[0].min(i[1]));
    }
    rect.sort_unstable();
    rect.reverse();
    for i in rect.windows(2) {
        if i[0] == i[1] {
            res += 1;
        } else {
            break;
        }
    }
    res
}

fn main() {}
