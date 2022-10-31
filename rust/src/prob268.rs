struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let len = nums.len() as f32;
        let expected_middle = len / 2.;
        let expected_last_val = len + 1.;
        let expected_sum = (expected_last_val * expected_middle) as i32;
        expected_sum - nums.iter().sum::<i32>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_cases() -> Vec<(Vec<i32>, i32)> {
        vec![
            (vec![3, 0, 1], 2),
            (vec![0, 1], 2),
            (vec![9, 6, 4, 2, 3, 5, 7, 0, 1], 8),
        ]
    }

    #[test]
    fn test_missing_number() {
        for (vec, expected) in test_cases() {
            assert_eq!(Solution::missing_number(vec), expected)
        }
    }
}
