use std::collections::{HashMap, HashSet};

pub fn roman_to_int(s: String) -> i32 {
    let mut s_iter = s.chars().into_iter().peekable();
    let mut double_chars: HashSet<char> = HashSet::new();
    double_chars.insert('C');
    double_chars.insert('X');
    double_chars.insert('I');
    // from_iter(vec!['C', 'X', 'I']);
    let roman_numerals = vec![
        ("M", 1000),
        ("CM", 900),
        ("D", 500),
        ("CD", 400),
        ("C", 100),
        ("XC", 90),
        ("L", 50),
        ("XL", 40),
        ("X", 10),
        ("IX", 9),
        ("V", 5),
        ("IV", 4),
        ("I", 1),
    ]
    .into_iter()
    .map(|(s, n)| (s.to_string(), n));
    let mut int_value: HashMap<String, i32> = HashMap::new();
    for (s, n) in roman_numerals {
        int_value.insert(s, n);
    }
    // from_iter(roman_numerals);

    let mut sum = 0;

    loop {
        match s_iter.next() {
            Some(c) => {
                if double_chars.contains(&c) {
                    match s_iter.peek() {
                        Some(&peek) => {
                            let mut joined = String::from(c);
                            joined.push(peek);
                            match int_value.get(&joined) {
                                Some(&n) => {
                                    s_iter.next();
                                    sum += n;
                                }
                                None => {
                                    sum += int_value.get(&String::from(c)).unwrap();
                                }
                            }
                        }
                        None => {
                            // int_value[&String::from(c)];
                            sum += int_value.get(&String::from(c)).unwrap();
                            return sum;
                        }
                    }
                } else {
                    sum += int_value.get(&String::from(c)).unwrap();
                }
            }
            None => {
                return sum;
            }
        }
    }
}

#[cfg(test)]
mod problem_tests {
    use super::*;

    #[test]
    fn test_roman_to_int1() {
        assert_eq!(roman_to_int(String::from("III")), 3);
    }
    #[test]
    fn test_roman_to_int2() {
        assert_eq!(roman_to_int(String::from("LVIII")), 58);
    }
    #[test]
    fn test_roman_to_int3() {
        assert_eq!(roman_to_int(String::from("MCMXCIV")), 1994);
    }
}
