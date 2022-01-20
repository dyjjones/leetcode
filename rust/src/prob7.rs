pub fn reverse(x: i32) -> i32 {
    let mut nums = vec![];
    let mut div = x;
    let mut rem;
    loop {
        rem = div % 10;
        div = div / 10;
        nums.push(rem);

        if div.abs() < 10 {
            if div != 0 {
                nums.push(div);
            }
            break;
        }
    }
    let mut reversed: i32 = 0;
    for (n, i) in nums.iter().zip((0..(nums.len())).rev()) {
        match 10_i32.checked_pow(i as _) {
            Some(m) => match m.checked_mul(*n) {
                Some(summation_piece) => match reversed.checked_add(summation_piece) {
                    Some(checked_add) => {
                        reversed = checked_add;
                    }
                    None => {
                        return 0;
                    }
                },
                None => {
                    return 0;
                }
            },
            None => {
                return 0;
            }
        }
    }
    reversed
}

#[cfg(test)]
mod problem_tests {
    use super::*;

    #[test]
    fn test_reverse() {
        assert_eq!(reverse(123), 321);
        assert_eq!(reverse(120), 21);
        assert_eq!(reverse(-123), -321);
        assert_eq!(reverse(-2147483641), -1463847412);
        assert_eq!(reverse(2147483647), 0);
        assert_eq!(reverse(1), 1);
        assert_eq!(reverse(1563847412), 0);
    }
}
