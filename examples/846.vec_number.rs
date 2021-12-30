// https://leetcode-cn.com/problems/valid-tic-tac-toe-state/
fn main() {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        if hand.len() % group_size as usize != 0 {
            return false;
        }
        let mut hand = hand;
        hand.sort_unstable();
        let mut cache = std::collections::HashMap::new();
        hand.iter().for_each(|&h| *cache.entry(h).or_insert(0) += 1);
        for &h in hand.iter() {
            if cache.get(&h) == None {
                continue;
            }
            for i in h..h + group_size {
                if cache.get(&i) == None {
                    return false;
                }
                let val = *cache.get(&i).unwrap();
                cache.insert(i, val - 1);
                if *cache.get(&i).unwrap() == 0 {
                    cache.remove(&i);
                }
            }
        }
        true
    }
}
