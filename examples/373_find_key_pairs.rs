use std::{cmp::min, collections::BinaryHeap};
pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
    let mut ans = Vec::new();
    let mut heap = BinaryHeap::new();
    (0..min(nums1.len(), k as usize)).for_each(|i| heap.push((-(nums1[i] + nums2[0]), i, 0)));
    while let Some((_, i, j)) = heap.pop() {
        ans.push(vec![nums1[i], nums2[j]]);
        if ans.len() as i32 == k {
            break;
        }
        if j + 1 < nums2.len() {
            heap.push((-(nums1[i] + nums2[j + 1]), i, j + 1));
        }
    }
    ans
}
fn main() {}
