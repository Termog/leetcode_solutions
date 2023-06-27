use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map = HashMap::with_capacity(nums.len());
        for num in nums {
            match map.get_mut(&num) {
                Some(count) => *count += 1,
                None => {let _ = map.insert(num,1);},
            } 
        }
        let mut nums2: Vec<(i32,u8)> = map.into_iter().collect();
        nums2.sort_unstable_by_key(|x| x.1);
        let skip = nums2.len() - k as usize;
        nums2[(nums2.len()-k as usize)..nums2.len()].into_iter().map(|x| x.0).collect()
    }
}
