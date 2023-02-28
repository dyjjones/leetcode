struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() == 1 {
            if nums[0] == target {
                return 0;
            }
            return -1;
        }
        if target < nums[0] || nums[nums.len() - 1] < target {
            return -1;
        }
        let mut lower = 0;
        let mut higher = nums.len() - 1;
        let mut index;
        let mut num: &i32;
        loop {
            index = lower + (higher - lower) / 2;
            num = &nums[index];
            if *num == target {
                return index as _;
            } else if lower == higher {
                return -1;
            } else if *num < target {
                lower = index + 1;
            } else {
                higher = index - 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search1() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 9;

        let output = Solution::search(nums, target);
        assert_eq!(output, 4)
    }
    #[test]
    fn test_search2() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 2;

        let output = Solution::search(nums, target);
        assert_eq!(output, -1)
    }
}
