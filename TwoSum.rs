use std::collections::HashMap;

impl Solution {
    pub fn two_sum(mut nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::with_capacity(nums.len());
        for (i,num) in nums.into_iter().enumerate() {
            if map.contains_key(&(target-num)) {
                return vec![*(map.get(&(target-num)).unwrap()),i as i32]
            }
            map.insert(num,i as i32);
        }
        //unreachable
        panic!();
    }
}
