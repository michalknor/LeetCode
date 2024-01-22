use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut seen: HashMap<char, usize> = HashMap::new();
        let mut result: usize = 0;
        let mut start = 0;

        for (end, character) in s.chars().enumerate() {
            if !seen.contains_key(&character) || seen.get(&character).unwrap() < &start {
                let current = end - start + 1;

                if current > result {
                    result = current;
                }

                seen.insert(character, end);

                continue;
            }
            
            start = seen.get(&character).unwrap() + 1;
            seen.insert(character, end);
        }
        
        result as i32
    }
}

pub fn tests() {
    assert_eq!(3, Solution::length_of_longest_substring("abcabcbb".to_string()));
    assert_eq!(1, Solution::length_of_longest_substring("bbbbb".to_string()));
    assert_eq!(3, Solution::length_of_longest_substring("pwwkew".to_string()));
    assert_eq!(3, Solution::length_of_longest_substring("dvdf".to_string()));
}