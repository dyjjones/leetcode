pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut strs_iter = strs.iter();
    let mut longest = strs_iter.next().unwrap().chars().collect::<Vec<char>>();

    for s in strs_iter {
        let s_len = s.len();
        if s_len < longest.len() {
            longest.truncate(s_len);
        }
        let zip = longest.iter().zip(s.chars()).enumerate();
        for (i, (&c1, c2)) in zip {
            if c1 != c2 {
                // cut longest at this index
                if i == 0 {
                    return String::from("");
                }
                longest.truncate(i);
                break;
            }
        }
    }

    longest.iter().collect::<String>()
}

#[cfg(test)]
mod problem_tests {
    use super::*;

    #[test]
    fn test_longest_common_prefix1() {
        assert_eq!(
            longest_common_prefix(
                vec!["flower", "flow", "flight"]
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
            ),
            "fl".to_string()
        );
    }
    #[test]
    fn test_longest_common_prefix2() {
        assert_eq!(
            longest_common_prefix(
                vec!["dog", "racecar", "car"]
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
            ),
            "".to_string()
        )
    }
    #[test]
    fn test_longest_common_prefix3() {
        assert_eq!(
            longest_common_prefix(
                vec!["ab", "a"]
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
            ),
            "a".to_string()
        )
    }
}
