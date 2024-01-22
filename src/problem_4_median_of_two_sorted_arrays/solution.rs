struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (mut i, mut j) = (0, 0);
        let (max_i, max_j) = (nums1.len(), nums2.len());

        let (first, second) = match (max_i + max_j) % 2 == 0 {
            true => ((max_i + max_j) / 2 - 1, (max_i + max_j) / 2),
            false => ((max_i + max_j) / 2, (max_i + max_j) / 2)
        };

        let (mut current_count, mut current) = (0, 0);
        let mut first_part = 0;

        loop {
            if i == max_i {
                current = nums2[j];
                j += 1;
            }
            else if j == max_j {
                current = nums1[i];
                i += 1;
            }
            else {
                match nums1[i] < nums2[j] {
                    true => {
                        current = nums1[i];
                        i += 1;
                    }
                    false => {
                        current = nums2[j];
                        j += 1;
                    }
                }
            }

            if current_count == first {
                first_part = current;
            }

            if current_count == second {
                return (current as f64 + first_part as f64) / 2.0;
            }

            current_count += 1;
        }
    }
}

pub fn tests() {
    assert_eq!(2.0, Solution::find_median_sorted_arrays(vec![1,3], vec![2]));
    assert_eq!(2.5, Solution::find_median_sorted_arrays(vec![1,2], vec![3,4]));
    assert_eq!(1.0, Solution::find_median_sorted_arrays(vec![], vec![1]));
}