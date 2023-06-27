impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
       let mut prods = vec![1;nums.len()];
       let mut prefix = nums[0];
       for i in 1..nums.len() {
           prods[i] *= prefix;
           prefix *= nums[i];
       }
       let mut postfix = nums[nums.len()-1];
       for i in (0..(nums.len()-1)).rev() {
           prods[i] *= postfix;
           postfix *= nums[i];
       }
       prods
    }
}
