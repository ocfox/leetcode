pub fn find_min_difference(time_points: Vec<String>) -> i32 {
    let mut time_points = time_points;
    let mut time_points: Vec<i32> = time_points.iter_mut().map(|x| x.replace(":", "").parse().unwrap()).collect();
    time_points.sort_unstable();
    let mut res = 1440;
    for split in time_points.windows(2) {
        res = res.min((split[0] - split[1]).abs());
    }
    res
}

fn main() {
    find_min_difference(vec!["00:00".to_string(),"23:59".to_string(),"00:00".to_string()]);
}
