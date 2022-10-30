struct Solution;

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut closest = None;
        let mut delta = i32::MAX;
        let mut a_i = 0_usize;
        let mut b_i = 1_usize;
        let last = nums.len() - 1;
        let mut c_i = last;

        let mut first;
        let mut second;
        let mut third;
        loop {
            first = &nums[a_i];
            second = &nums[b_i];
            third = &nums[c_i];
            let sum = first + second + third;
            if sum < target {
                if target - sum < delta {
                    closest = Some(sum);
                    delta = target - sum;
                }
                // decrement c_i if not next to b_i
                if b_i + 1 == c_i {
                    // reset positions or return
                    if a_i + 1 == b_i {
                        return closest.unwrap();
                    } else {
                        a_i += 1;
                        b_i = a_i + 1;
                        c_i = last;
                    }
                } else {
                    b_i += 1;
                }
            } else if sum == target {
                return target;
            } else if target < sum {
                if sum - target < delta {
                    closest = Some(sum);
                    delta = sum - target;
                }
                if b_i + 1 == c_i {
                    // reset positions or return
                    if a_i + 1 == b_i {
                        return closest.unwrap();
                    } else {
                        a_i += 1;
                        b_i = a_i + 1;
                        c_i = last;
                    }
                } else {
                    c_i -= 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_sum_closest1() {
        assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
    }
    #[test]
    fn test_three_sum_closest2() {
        assert_eq!(Solution::three_sum_closest(vec![0, 0, 0], 1), 0);
    }
    #[test]
    fn test_three_sum_closest3() {
        assert_eq!(Solution::three_sum_closest(vec![-3, -2, -5, 3, -4], -1), -2);
    }
    #[test]
    fn test_three_sum_closest4() {
        assert_eq!(
            Solution::three_sum_closest(vec![-111, -111, 3, 6, 7, 16, 17, 18, 19], 13),
            16
        );
    }
}
