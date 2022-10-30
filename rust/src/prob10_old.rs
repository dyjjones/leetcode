// use std::str::pattern;

// use std::iter::repeat;
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum RegExpr {
    AnyChar,
    ZeroOrMoreAnyChar,
    Literal(char),
    ZeroOrMoreLiteral(char),
}

impl RegExpr {
    fn matches(self, ch: char) -> bool {
        match self {
            Self::AnyChar => true,
            Self::ZeroOrMoreAnyChar => true,
            Self::Literal(c) => c == ch,
            Self::ZeroOrMoreLiteral(c) => c == ch,
        }
    }

    // fn generate_
}

// I could have done it like this, but I felt
// that `instance` was less explicit and more confusing
// struct GenericGroup<T> {
//     instance: T,
//     occurrences: i32,
//     original_index: usize
// }
// type ParseGroup = GenericGroup<char>;
// type RegexGroup = GenericGroup<RegExpr>;
struct ParseGroup {
    ch: char,
    occurrences: i32,
    original_index: usize,
}
// struct Parseable(Vec<(char, i32)>);
struct Parseable(Vec<ParseGroup>);

impl Parseable {
    fn get(Parseable(vec): Parseable, group_index: usize, sub_index: usize) -> Option<char> {
        // let remaining =
        // for (ch, i) in vec {

        // }
        match vec.get(group_index) {
            Some(&ParseGroup {
                ch, occurrences, ..
            }) => {
                if sub_index < occurrences as _ {
                    return Some(ch);
                } else {
                    return None;
                }
            }
            None => None,
        }
    }
    fn get_with_original_index(Parseable(vec): Parseable, original_index: usize) -> Option<char> {
        let mut original_index = original_index as i32;
        for ParseGroup {
            ch: ch,
            occurrences: occurrences,
            ..
        } in vec
        {
            if original_index < occurrences {
                return Some(ch);
            }
            original_index -= occurrences;
        }
        return None;
    }
}

struct RegexGroup {
    expression: RegExpr,
    occurrences: i32,
    original_index: usize,
}
struct RegularExpression(Vec<RegexGroup>);

impl RegularExpression {
    fn matches(
        RegularExpression(regex_vec): RegularExpression,
        parseable @ Parseable(parsed_string_vec): Parseable,
    ) -> bool {
        if regex_vec.is_empty() {
            // ([], []) => {return true;}
            // ([], _) => {return false;}
            return parsed_string_vec.is_empty();
        }
        let mut parseable_index = 0_usize;
        let mut parseable_subindex = 0_usize;
        let mut pattern_index = 0_usize;
        let mut pattern_subindex = 0_usize;

        loop {
            let current_expr_group_opt = regex_vec.get(pattern_index);
            match current_expr_group_opt {
                Some(&current_expr_group) => {}
                None => {
                    // not sure if this is where to do this part...
                    // let's go anyway
                    // parsed_string must be empty if this is reached,
                    // otherwise return false
                    // if parsed_str
                    if parsed_string_vec.len() - 1 == parseable_index
                        && parsed_string_vec.last().occurrences
                    {}
                }
            }
            // match regex_vec {
            //     [] => {}
            //     [(expr, i)] => {}
            //     [(expr, i), tail @ ..] => {}
            // }
        }
        false
    }
}

fn group_subpatterns(v: Vec<RegExpr>) -> Vec<(RegExpr, i32)> {
    let mut peekable_vec = v.iter().peekable();
    let mut groups = vec![];
    let mut current;
    let mut length = 0;
    match peekable_vec.peek() {
        Some(&&subexpr) => {
            current = subexpr;
        }
        None => {
            return groups;
        }
    }
    for &c in peekable_vec {
        if current == c {
            length += 1;
        } else {
            // let repeated = std::iter::repeat(current).take(length).collect::<Vec<_>>();
            // groups.push(repeated);
            groups.push((current, length));
            length = 1;
            current = c;
        }
    }
    // let repeated = std::iter::repeat(current).take(length).collect::<Vec<_>>();
    // groups.push(repeated);
    groups.push((current, length));

    return groups;
}

fn group_chars(s: String) -> Vec<(char, i32)> {
    let mut chars = s.chars().peekable();
    let mut groups = vec![];
    let mut current;
    let mut length = 0;
    match chars.peek() {
        Some(&c) => {
            current = c;
        }
        None => {
            return groups;
        }
    }
    for c in s.chars() {
        if current == c {
            length += 1;
        } else {
            // let repeated = std::iter::repeat(current).take(length).collect::<String>();
            // groups.push(repeated);
            groups.push((current, length));
            length = 1;
            current = c;
        }
    }
    // let repeated = std::iter::repeat(current).take(length).collect::<String>();
    // groups.push(repeated);
    groups.push((current, length));

    return groups;
}

fn pattern_to_sub_patterns(p: String) -> Vec<RegExpr> {
    let mut pattern_consumer = p.chars().peekable();
    let mut patterns = vec![];

    loop {
        match pattern_consumer.next() {
            None => {
                return patterns;
            }
            Some('.') => match pattern_consumer.peek() {
                Some('*') => {
                    pattern_consumer.next(); //advance it
                    patterns.push(RegExpr::ZeroOrMoreAnyChar);
                }
                _ => {
                    patterns.push(RegExpr::AnyChar);
                }
            },
            Some(p_char) => match pattern_consumer.peek() {
                Some('*') => {
                    pattern_consumer.next();
                    patterns.push(RegExpr::ZeroOrMoreLiteral(p_char));
                }
                _ => {
                    patterns.push(RegExpr::Literal(p_char));
                }
            },
        }
    }
}

struct Solution;

impl Solution {
    pub fn is_match(s: String, pattern: String) -> bool {
        // you need to look ahead for each literal/. * expression
        // to an anchor, recurse with that, if its not valid, look
        // for the next anchor in the string
        /* Possible branches:
          [a-z]* => can go to end of literal group
          .* => can go to end of str
        */
        println!("START is_match");
        // let mut str_consumer = s.chars().peekable();

        let mut char_groups = group_chars(s);
        let mut pattern_groups = pattern_to_sub_patterns(pattern);
        let mut sub_pattern: RegExpr;
        let mut index: usize;
        // RegularExpression()
        fn loop_string_for_patterns(
            substrings: Vec<(char, i32)>,
            pattern_groups: Vec<(RegExpr, i32)>,
        ) -> bool {
            let mut substr_index: usize = 0;
            let mut pattern_groups_index: usize = 0;
            let mut pattern_group_subindex: usize = 0;
            loop {
                match pattern_groups.get(pattern_groups_index) {
                    Some(&(current_pattern, current_pattern_group_size)) => {
                        // idk
                    }
                    None => {
                        return false;
                    }
                }
            }
            unimplemented!()
        }

        return loop_string_for_patterns(char_groups, group_subpatterns(pattern_groups));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_subpatterns1() {
        assert_eq!(
            group_subpatterns(Vec::<RegExpr>::new()),
            Vec::<(RegExpr, i32)>::new()
        );
        assert_eq!(
            group_subpatterns(vec![RegExpr::Literal('a'), RegExpr::Literal('a')]),
            vec![(RegExpr::Literal('a'), 2)]
        );
        assert_eq!(
            group_subpatterns(vec![
                RegExpr::Literal('a'),
                RegExpr::AnyChar,
                RegExpr::Literal('a')
            ]),
            vec![
                (RegExpr::Literal('a'), 1),
                (RegExpr::AnyChar, 1),
                (RegExpr::Literal('a'), 1)
            ]
        );
    }
    #[test]
    fn test_group_chars1() {
        assert_eq!(group_chars("".to_string()), Vec::<(char, i32)>::new());
    }
    #[test]
    fn test_group_chars2() {
        assert_eq!(group_chars("a".to_string()), vec![('a', 1)]);
    }
    #[test]
    fn test_str_group_by_char3() {
        assert_eq!(
            group_chars("aabcccdeff".to_string()),
            vec![('a', 2), ('b', 1), ('c', 3), ('d', 1), ('e', 1), ('f', 2)]
        );
    }

    #[test]
    fn test_pattern_to_subpatterns_simple() {
        assert_eq!(
            pattern_to_sub_patterns("".to_string()),
            Vec::<RegExpr>::new()
        );
        assert_eq!(
            pattern_to_sub_patterns("a".to_string()),
            vec![RegExpr::Literal('a')]
        );
        assert_eq!(
            pattern_to_sub_patterns("a*".to_string()),
            vec![RegExpr::ZeroOrMoreLiteral('a')]
        );
        assert_eq!(
            pattern_to_sub_patterns(".".to_string()),
            vec![RegExpr::AnyChar]
        );
        assert_eq!(
            pattern_to_sub_patterns(".*".to_string()),
            vec![RegExpr::ZeroOrMoreAnyChar]
        );
    }

    #[test]
    fn test_pattern_to_subpatterns_complex() {
        assert_eq!(
            pattern_to_sub_patterns("a*bc.*".to_string()),
            vec![
                RegExpr::ZeroOrMoreLiteral('a'),
                RegExpr::Literal('b'),
                RegExpr::Literal('c'),
                RegExpr::ZeroOrMoreAnyChar
            ]
        );
        assert_eq!(
            pattern_to_sub_patterns("a*bc.*.".to_string()),
            vec![
                RegExpr::ZeroOrMoreLiteral('a'),
                RegExpr::Literal('b'),
                RegExpr::Literal('c'),
                RegExpr::ZeroOrMoreAnyChar,
                RegExpr::AnyChar
            ]
        );
        assert_eq!(
            pattern_to_sub_patterns("a*bc.*.*".to_string()),
            vec![
                RegExpr::ZeroOrMoreLiteral('a'),
                RegExpr::Literal('b'),
                RegExpr::Literal('c'),
                RegExpr::ZeroOrMoreAnyChar,
                RegExpr::ZeroOrMoreAnyChar
            ]
        );
        assert_eq!(
            pattern_to_sub_patterns("aa*bb*.*a*a.*".to_string()),
            vec![
                RegExpr::Literal('a'),
                RegExpr::ZeroOrMoreLiteral('a'),
                RegExpr::Literal('b'),
                RegExpr::ZeroOrMoreLiteral('b'),
                RegExpr::ZeroOrMoreAnyChar,
                RegExpr::ZeroOrMoreLiteral('a'),
                RegExpr::Literal('a'),
                RegExpr::ZeroOrMoreAnyChar
            ]
        );
    }

    #[test]
    fn test_is_match_simple() {
        assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
        assert_eq!(Solution::is_match("".to_string(), "a*".to_string()), true);
        assert_eq!(Solution::is_match("a".to_string(), "a*".to_string()), true);
        assert_eq!(Solution::is_match("aa".to_string(), "a*".to_string()), true);
        assert_eq!(
            Solution::is_match("aaa".to_string(), "a*".to_string()),
            true
        );
        assert_eq!(Solution::is_match("ab".to_string(), ".*".to_string()), true);
    }

    #[test]
    fn test_is_match_complex1() {
        assert_eq!(
            Solution::is_match("aabccd".to_string(), "a*bc.*".to_string()),
            true
        );
    }
    #[test]
    fn test_is_match_complex2() {
        assert_eq!(
            Solution::is_match("aabccd".to_string(), "a*bc.*.".to_string()),
            false
        );
    }
    #[test]
    fn test_is_match_complex3() {
        assert_eq!(
            Solution::is_match("aabccd".to_string(), "a*bc.*a".to_string()),
            false
        );
    }
    #[test]
    fn test_is_match_complex4() {
        assert_eq!(
            Solution::is_match("aabccde".to_string(), "a*bc.*.*".to_string()),
            true
        );
    }
    #[test]
    fn test_is_match_complex5() {
        assert_eq!(
            Solution::is_match("aabccd".to_string(), "a*bx*c.*".to_string()),
            true
        );

        // assert_eq!(is_match("aabccd", "a*bx*c.*"), true);
        // assert_eq!(what_happens("aabb".to_string()), "bb");
    }

    #[test]
    fn test_is_match_complex6() {
        assert_eq!(
            Solution::is_match("aaa".to_string(), "ab*a*c*a".to_string()),
            true
        );
    }

    #[test]
    fn test_is_match_complex7() {
        assert_eq!(
            Solution::is_match("aaabcaaa".to_string(), "aa*b.*a*a".to_string()),
            true
        );
    }

    #[test]
    fn test_is_match_complex8() {
        assert_eq!(
            Solution::is_match("aaabbacaabbaa".to_string(), "aa*bb*.*a*a.*".to_string()),
            true
        );
    }

    #[test]
    fn test_complex_10() {
        assert!(Solution::is_match(
            String::from("abcaaaaaaabaabcabac"),
            String::from(".*ab.a.*a*a*.*b*b*")
        ));
    }
}

/* aaabcaaa
   aa*b.*a*a

   aaabbacaabbaa
   aa*bb*.*a*a.*
*/
