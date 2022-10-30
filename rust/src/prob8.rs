struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        // let handling_white_space = true;
        // per the problem description, only white
        // space to be consider is the space char
        let to_remove = " ";
        let mut s = s.trim_start_matches(to_remove).to_string();
        let mut is_negative = false;
        match s.chars().next() {
            Some('+') => {
                s = s[1..].to_string();
            }
            Some('-') => {
                s = s[1..].to_string();
                is_negative = true;
            }
            None => {
                // empty string, besides front white space
                return 0;
            }
            _ => (),
        }
        let num_str = s
            .chars()
            .take_while(|c| c.is_ascii_digit())
            .collect::<String>();

        if num_str.len() == 0 {
            return 0;
        }
        let number = match num_str.parse::<i32>() {
            Ok(n) => match is_negative {
                true => -n,
                false => n,
            },
            Err(_) => match is_negative {
                true => i32::MIN,
                false => i32::MAX,
            },
        };
        number
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_atoi() {
        assert_eq!(Solution::my_atoi("  37.blah".to_string()), 37);
        assert_eq!(Solution::my_atoi("  2147483648fg".to_string()), 2147483647);
        assert_eq!(Solution::my_atoi("0032".to_string()), 32);
        assert_eq!(Solution::my_atoi("d0032".to_string()), 0);
        assert_eq!(Solution::my_atoi("".to_string()), 0);
    }
}
