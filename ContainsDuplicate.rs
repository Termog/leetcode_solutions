use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = HashSet::with_capacity(nums.len());
        for i in nums {
            match set.insert(i) {
                false => return true,
                _ => (),
            }
        }
        false
    }
}
