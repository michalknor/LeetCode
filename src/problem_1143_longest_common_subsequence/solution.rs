use std::time::Instant;


struct Solution;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let text1: Vec<char> = text1.chars().collect::<Vec<char>>();
        let text2: Vec<char> = text2.chars().collect::<Vec<char>>();
        
        let mut memory: Vec<Vec<i32>> = vec![vec![0; text2.len()+1]; text1.len()+1];

        for i in 0..text1.len() {
            for j in 0..text2.len() {
                if text1[i] == text2[j] {
                    memory[i+1][j+1] = memory[i+1][j+1].max(memory[i][j]+1);
                    continue;
                }
                
                memory[i+1][j+1] = memory[i+1][j+1].max(memory[i+1][j]).max(memory[i][j+1]);
            }
        }

        memory[text1.len()][text2.len()]
    }
}

pub fn tests() {
    let now = Instant::now();
    assert_eq!(3, Solution::longest_common_subsequence("abcde".to_string(), "ace".to_string()));
    assert_eq!(3, Solution::longest_common_subsequence("abc".to_string(), "abc".to_string()));
    assert_eq!(0, Solution::longest_common_subsequence("abc".to_string(), "def".to_string()));
    assert_eq!(1, Solution::longest_common_subsequence("bl".to_string(), "yby".to_string()));
    assert_eq!(2, Solution::longest_common_subsequence("ezupkr".to_string(), "ubmrapg".to_string()));
    assert_eq!(3, Solution::longest_common_subsequence("ylqpejqbalahwr".to_string(), "yrkzavgdmdgtqpg".to_string()));
    assert_eq!(6, Solution::longest_common_subsequence("yzebsbuxmtcfmtodclszgh".to_string(), "ejevmhcvshclydqrulwbyha".to_string()));
    assert_eq!(11, Solution::longest_common_subsequence("fcvqfcnglshwpgxujwrylqzejmdubkxs".to_string(), "bctsfwdelkdqzshupmrufyxklsjurevip".to_string()));
    assert_eq!(14, Solution::longest_common_subsequence("opmtqvejqvudezchsloxizynabehqbyzknunobehkzqtkt".to_string(), "srwbovohkvqhwrwvizebsrszcxepqrenilmvadqxuncpwhe".to_string()));
    println!("{}: {}", "Elapsed", format!("{:.2?}", now.elapsed()));

}