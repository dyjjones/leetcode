struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        // let x_str = x.to_string();
        // let x_reverse = x_str.chars().rev().collect::<String>();
        // x_str == x_reverse

        let mut nums = vec![];
        let mut div = x;
        let mut rem;
        loop {
            rem = div % 10;
            div /= 10;
            nums.push(rem);

            if div.abs() < 10 {
                if div != 0 {
                    nums.push(div);
                }
                break;
            }
        }
        let mut reversed = nums.clone();
        reversed.reverse();
        nums == reversed
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert!(Solution::is_palindrome(121));
        assert!(!Solution::is_palindrome(-123));
        assert!(!Solution::is_palindrome(10));
        assert!(Solution::is_palindrome(0));
        assert!(Solution::is_palindrome(1));
        assert!(Solution::is_palindrome(1000000001));
        assert!(Solution::is_palindrome(1567651));
    }
}
