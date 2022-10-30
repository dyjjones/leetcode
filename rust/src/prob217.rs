use std::collections::HashSet;

struct Solution;

/* Two solutions considered:
 * length(nums) = set(length(nums))
 * iterate through once and insert into array if not in, otherwise return true
 */
impl Solution {
    pub fn contains_duplicate1(nums: Vec<i32>) -> bool {
        let nums_set = nums.iter().collect::<HashSet<_>>();
        nums_set.len() != nums.len()
    }
    pub fn contains_duplicate2(nums: Vec<i32>) -> bool {
        let mut hs = HashSet::with_capacity(nums.len());
        for n in nums {
            if hs.contains(&n) {
                return true;
            } else {
                hs.insert(n);
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_cases() -> Vec<(Vec<i32>, bool)> {
        vec![
            (vec![1, 2, 3, 1], true),
            (vec![1, 2, 3, 4], false),
            (vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2], true),
        ]
    }

    #[test]
    fn test_contains_duplicate1() {
        for (expected, got) in test_cases() {
            assert_eq!(Solution::contains_duplicate1(expected), got);
        }
    }

    #[test]
    fn test_contains_duplicate2() {
        for (expected, got) in test_cases() {
            assert_eq!(Solution::contains_duplicate1(expected), got);
        }
    }
}
