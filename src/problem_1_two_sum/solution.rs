use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut table: HashMap<i32, usize> = HashMap::new();
        
        for (i, number) in numbers.iter().enumerate() {
            if let Some(number_cached) = table.get(&(target - number)) {
                return vec![*number_cached as i32, i as i32];
            }
            
            table.insert(*number, i);
        }

        vec![]
    }
}

pub fn tests() {
    assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
    assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4], 6));
    assert_eq!(vec![0, 1], Solution::two_sum(vec![3, 3], 6));
}