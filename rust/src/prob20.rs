use std::collections::HashMap;

pub fn is_valid(s: String) -> bool {
    let mut stack = vec![];
    let mut pairs: HashMap<char, char> = HashMap::new();
    pairs.insert(')', '(');
    pairs.insert('}', '{');
    pairs.insert(']', '[');

    for c in s.chars().into_iter() {
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

#[cfg(test)]
mod problem_tests {
    use super::*;

    #[test]
    fn test_is_valid1() {
        assert_eq!(is_valid("()".to_string()), true);
    }
    #[test]
    fn test_is_valid2() {
        assert_eq!(is_valid("()[]{}".to_string()), true);
    }
    #[test]
    fn test_is_valid3() {
        assert_eq!(is_valid("(]".to_string()), false);
    }
    #[test]
    fn test_is_valid4() {
        assert_eq!(is_valid("()[({{}}())]".to_string()), true);
    }
}
