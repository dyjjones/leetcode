struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        fn is_palindrome(s: &str) -> bool {
            s.as_bytes().iter().eq(s.as_bytes().iter().rev())
        }
        if s.len() < 2 {
            return s;
        }
        let total_len = s.len();
        for length in (1..=s.len()).rev() {
            for i in 0..=(total_len - length) {
                let substr = &s[i..i + length];
                if is_palindrome(substr) {
                    return substr.to_string();
                }
            }
        }
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_palindrome() {
        assert_eq!(
            Solution::longest_palindrome("babad".to_string()),
            "bab".to_string()
        );
        assert_eq!(
            Solution::longest_palindrome("cbbd".to_string()),
            "bb".to_string()
        );
        assert_eq!(Solution::longest_palindrome("".to_string()), "".to_string());
        assert_eq!(
            Solution::longest_palindrome("a".to_string()),
            "a".to_string()
        );
        assert_eq!(
            Solution::longest_palindrome("ac".to_string()),
            "a".to_string()
        );
    }
}
