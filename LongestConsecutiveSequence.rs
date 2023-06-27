impl Solution {
    pub fn longest_consecutive(mut nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        nums.sort_unstable();
        nums.dedup();
        println!("{:?}",nums);
        let mut nums = nums.into_iter();
        let mut last = nums.next().unwrap();
        let mut count = 1;
        let mut max = 1;
        for num in nums {
            println!("{}:{}",last,num);
            if num == last+1 {
                count += 1;
            } else {
                if count > max {
                    max = count;
                }
                count = 1;
            }
            last = num;
        }
        if count > max {
            count
        } else {
            max
        }
    }
}
