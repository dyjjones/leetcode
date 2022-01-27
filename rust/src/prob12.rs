pub fn int_to_roman(num: i32) -> String {
    let mut builder = String::new();

    let mut div;
    let mut rem = num;

    let roman_numerals = vec![
        ('M', 1000, Some(("CM", 900))),
        ('D', 500, Some(("CD", 400))),
        ('C', 100, Some(("XC", 90))),
        ('L', 50, Some(("XL", 40))),
        ('X', 10, Some(("IX", 9))),
        ('V', 5, Some(("IV", 4))),
        ('I', 1, None),
    ];

    for (c, size, optional_adder) in roman_numerals {
        div = rem / size;
        rem = rem % size;
        // let _ @ (div, rem) = (rem / 1000, rem % 1000);
        builder.push_str(&(std::iter::repeat(c).take(div as _).collect::<String>()) as _);
        match optional_adder {
            Some((s, subtractor)) => {
                if subtractor <= rem {
                    builder.push_str(s);
                    rem -= subtractor;
                }
            }
            None => {}
        }
    }

    builder
}

#[cfg(test)]
mod problem_tests {
    use super::*;

    #[test]
    fn test_int_to_roman() {
        assert_eq!(int_to_roman(3), "III");
        assert_eq!(int_to_roman(58), "LVIII");
        assert_eq!(int_to_roman(1994), "MCMXCIV");
    }
}
