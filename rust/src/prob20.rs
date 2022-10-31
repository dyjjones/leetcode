use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];
        let mut pairs: HashMap<char, char> = HashMap::new();
        pairs.insert(')', '(');
        pairs.insert('}', '{');
        pairs.insert(']', '[');

        for c in s.chars() {
            match c {
                '(' | '[' | '{' => {
                    stack.push(c);
                }
                _ => match stack.pop() {
                    None => {
                        return false;
                    }
                    Some(popped) => {
                        if pairs[&c] != popped {
                            return false;
                        }
                    }
                },
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid1() {
        assert!(Solution::is_valid("()".to_string()));
    }
    #[test]
    fn test_is_valid2() {
        assert!(Solution::is_valid("()[]{}".to_string()));
    }
    #[test]
    fn test_is_valid3() {
        assert!(!Solution::is_valid("(]".to_string()));
    }
    #[test]
    fn test_is_valid4() {
        assert!(Solution::is_valid("()[({{}}())]".to_string()));
    }
}
