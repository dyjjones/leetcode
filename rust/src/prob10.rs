// This solution was my first working solution
// with times of 533ms and 518 ms.
// I figured I could improve that time significantly
// so I modified it to the uncommented solution below
// and got 25 ms and 15 ms!

// pub fn is_match(s: String, pattern: String) -> bool {
//     if pattern.is_empty() {
//         return s.is_empty();
//     }

//     let s_vec = s.chars().collect::<Vec<char>>();
//     let p_vec = pattern.chars().collect::<Vec<char>>();
//     let first_match = !s.is_empty() && (p_vec[0] == s_vec[0] || p_vec[0] == '.');

//     if 2 <= pattern.len() && p_vec[1] == '*' {
//         // recurse nongreedily
//         // // if that failed, recurse consuming s.head
//         is_match(s, p_vec[2..].iter().collect())
//             || (first_match && is_match(s_vec[1..].iter().collect(), pattern))
//     } else {
//         first_match && is_match(s_vec[1..].iter().collect(), p_vec[1..].iter().collect())
//     }
// }

pub fn is_match(s: String, p: String) -> bool {
    fn rec(text: &[char], pattern: &[char]) -> bool {
        if pattern.is_empty() {
            text.is_empty()
        } else {
            let first_match = !text.is_empty() && (pattern[0] == '.' || pattern[0] == text[0]);
            if pattern.len() >= 2 && pattern[1] == '*' {
                rec(text, &pattern[2..]) || first_match && rec(&text[1..], pattern)
            } else {
                first_match && rec(&text[1..], &pattern[1..])
            }
        }
    }
    rec(
        &s.chars().collect::<Vec<_>>(),
        &p.chars().collect::<Vec<_>>(),
    )
}

#[cfg(test)]
mod problem_tests {
    use super::*;

    #[test]
    fn test_is_match_simple() {
        assert_eq!(is_match("aa".to_string(), "a".to_string()), false);
        assert_eq!(is_match("".to_string(), "a*".to_string()), true);
        assert_eq!(is_match("a".to_string(), "a*".to_string()), true);
        assert_eq!(is_match("aa".to_string(), "a*".to_string()), true);
        assert_eq!(is_match("aaa".to_string(), "a*".to_string()), true);
        assert_eq!(is_match("ab".to_string(), ".*".to_string()), true);
    }

    #[test]
    fn test_is_match_complex1() {
        assert_eq!(is_match("aabccd".to_string(), "a*bc.*".to_string()), true);
    }
    #[test]
    fn test_is_match_complex2() {
        assert_eq!(is_match("aabccd".to_string(), "a*bc.*.".to_string()), true);
    }
    #[test]
    fn test_is_match_complex3() {
        assert_eq!(is_match("aabccd".to_string(), "a*bc.*a".to_string()), false);
    }
    #[test]
    fn test_is_match_complex4() {
        assert_eq!(
            is_match("aabccde".to_string(), "a*bc.*.*".to_string()),
            true
        );
    }
    #[test]
    fn test_is_match_complex5() {
        assert_eq!(is_match("aabccd".to_string(), "a*bx*c.*".to_string()), true);

        // assert_eq!(is_match("aabccd", "a*bx*c.*"), true);
        // assert_eq!(what_happens("aabb".to_string()), "bb");
    }

    #[test]
    fn test_is_match_complex6() {
        assert_eq!(is_match("aaa".to_string(), "ab*a*c*a".to_string()), true);
    }

    #[test]
    fn test_is_match_complex7() {
        assert_eq!(
            is_match("aaabcaaa".to_string(), "aa*b.*a*a".to_string()),
            true
        );
    }

    #[test]
    fn test_is_match_complex8() {
        assert_eq!(
            is_match("aaabbacaabbaa".to_string(), "aa*bb*.*a*a.*".to_string()),
            true
        );
    }

    #[test]
    fn test_complex_10() {
        assert!(is_match(
            String::from("abcaaaaaaabaabcabac"),
            String::from(".*ab.a.*a*a*.*b*b*")
        ));
    }
}
