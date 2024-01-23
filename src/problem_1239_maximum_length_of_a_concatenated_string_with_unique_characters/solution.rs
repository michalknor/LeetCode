use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn max_length(arr: Vec<String>) -> i32 {
        let mut stack: Vec<(HashSet<char>, usize)> = Vec::new();

        let mut max_len: usize = 0;

        for i in 0..arr.len() {
            stack.push((HashSet::new(), i));
        }

        'outer: while let Some((mut unique_characters, current_index)) = stack.pop() {

            for character in arr[current_index].chars() {
                if unique_characters.contains(&character) {
                    continue 'outer;
                }

                unique_characters.insert(character);
            }

            if unique_characters.len() > max_len {
                max_len = unique_characters.len()
            }

            for i in current_index+1..arr.len() {
                stack.push((unique_characters.clone(), i));
            }
        }

        max_len as i32
    }
}

pub fn tests() {
    assert_eq!(4, Solution::max_length(vec!["un".to_string(),"iq".to_string(),"ue".to_string()]));
    assert_eq!(6, Solution::max_length(vec!["cha".to_string(),"r".to_string(),"act".to_string(),"ers".to_string()]));
    assert_eq!(26, Solution::max_length(vec!["abcdefghijklmnopqrstuvwxyz".to_string()]));
}