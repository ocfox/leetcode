fn main() {
    pub fn is_rectangle_cover(rectangles: Vec<Vec<i32>>) -> bool {
        let mut points = Vec::new();

        let mut regle = rectangles.clone();
        regle.sort();
        for x in 0..regle.len() - 1 {
            if regle[x] == regle[x + 1] {
                return false;
            }
        }
        let mut area = 0;
        for rect in rectangles {
            area += (rect[2] - rect[0]) * (rect[3] - rect[1]);
            points.push(vec![rect[0], rect[1]]);
            points.push(vec![rect[0], rect[3]]);
            points.push(vec![rect[2], rect[1]]);
            points.push(vec![rect[2], rect[3]]);
        }
        points.sort();

        for i in (1..points.len() - 1).rev() {
            if points[i] == points[i - 1] {
                points.remove(i - 1);
                points.remove(i - 1);
            }
        }
        if points.len() == 4 {
            if area == (points[3][1] - points[0][1]) * (points[3][0] - points[0][0]) {
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}
