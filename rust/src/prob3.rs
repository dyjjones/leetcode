use std::collections::HashMap;

pub fn length_of_longest_substring(s: String) -> i32 {
    if s.is_empty() {
        return 0;
    }
    //let mut set = HashSet::new();
    let mut map = HashMap::new();
    let mut longest = 0;
    let mut counter = 0;
    let mut lower_index = 0;
    // let mut current_window;

    for (index, c) in s.chars().enumerate() {
        match map.get(&c) {
            Some(&old_index) => {
                // check if it's inside the current window
                if old_index < lower_index {
                    // same as None match below
                    counter += 1;

                    if longest < counter {
                        longest = counter;
                    }
                } else {
                    lower_index = old_index + 1;
                    counter = index - lower_index + 1;
                }
            }
            None => {
                counter += 1;

                if longest < counter {
                    longest = counter;
                }
            }
        }
        map.insert(c, index);
    }
    longest as _
}

fn test(s: &str, count: i32) {
    assert_eq!(length_of_longest_substring(String::from(s)), count);
}

#[cfg(test)]
mod problem_tests {
    use super::*;

    #[test]
    fn test_length_of_longest_substring() {
        test("abcabcbb", 3);
        test("pwwkew", 3);
        test("bbbbb", 1);
        test("b", 1);
        test("dvdf", 3);
        test("rdvdabc", 5);
        test("rdvdrabc", 6);
        test("rdvdrabcc", 6);
    }
}
