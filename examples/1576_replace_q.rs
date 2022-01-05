fn main() {
    pub fn modify_string(s: String) -> String {
        let mut vs = s.chars().collect::<Vec<char>>();
        for i in 0..vs.len() {
            if vs[i] == '?' {
                let mut nc = 'a';
                while i != 0 && vs[i - 1] == nc || i != vs.len() - 1 && vs[i + 1] == nc {
                    nc = (nc as u8 + 1) as char;
                }
                vs[i] = nc;
            }
        }
        vs.iter().collect()
    }
}
