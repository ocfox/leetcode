pub fn slowest_key(release_times: Vec<i32>, keys_pressed: String) -> char {
    let keys_pressed: Vec<char> = keys_pressed.chars().collect();
    let (mut time, mut res) = (release_times[0], keys_pressed[0]);
    for i in 1..release_times.len() {
        if release_times[i] - release_times[i - 1] > time {
            time = release_times[i] - release_times[i - 1];
            res = keys_pressed[i];
        }
        if release_times[i] - release_times[i - 1] == time {
            res = keys_pressed[i].max(res);
        }
    }
    res
}

fn main() {
    let times = vec![4, 5, 7, 23, 32];
    println!("{}", slowest_key(times, "iemcs".to_string()));
}
