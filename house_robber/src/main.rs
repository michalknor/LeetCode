struct Solution;

impl Solution {
    pub fn rob(numbers: Vec<i32>) -> i32 {
        if numbers.len() == 1 {
            return numbers[0];
        }

        let mut stack: Vec<(usize, i32)> = vec![(0, numbers[0])];

        if numbers[1] > numbers[0] {
            stack.push((1, numbers[1]));
        }

        let mut max_loot: Vec<i32> = vec![-1; numbers.len()];
        
        while !stack.is_empty() {
            let (index, loot) = stack.pop().unwrap();

            if loot <= max_loot[index] {
                continue;
            }

            max_loot[index] = loot;

            if index >= numbers.len() - 2 {
                continue;
            }

            stack.push((index+2, loot + numbers[index+2]));

            if index >= numbers.len() - 3 {
                continue;
            }
            
            if numbers[index+3] > numbers[index+2] {
                stack.push((index+3, loot + numbers[index+3]));
            }
        }

        println!("{:?}", max_loot);

        if max_loot[numbers.len() - 1] > max_loot[numbers.len() - 2] {
            return max_loot[numbers.len() - 1]
        }
        
        max_loot[numbers.len() - 2]
    }
}

fn main() {
    assert_eq!(4, Solution::rob(vec![1,2,3,1]));
    assert_eq!(12, Solution::rob(vec![2,7,9,3,1]));
    assert_eq!(22, Solution::rob(vec![11,3,1,11,1]));
    assert_eq!(0, Solution::rob(vec![0]));
    assert_eq!(2, Solution::rob(vec![1,2]));
    assert_eq!(1, Solution::rob(vec![0,0,0,1]));
}