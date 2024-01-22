use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut not_seen: HashSet<usize> = (1..nums.len()+1).collect();

        let mut duplicate = 0;

        for num in &nums {
            if not_seen.contains(&(*num as usize)) {
                not_seen.remove(&(*num as usize));
            }
            else {
                duplicate = *num;
            }
        }

        if let Some(item) = not_seen.iter().next() {
            return vec![duplicate, *item as i32];
        }
        
        vec![0, 0]
    }
}

pub fn tests() {
    assert_eq!(vec![2,3], Solution::find_error_nums(vec![1,2,2,4]));
    assert_eq!(vec![1,2], Solution::find_error_nums(vec![1,1]));
}