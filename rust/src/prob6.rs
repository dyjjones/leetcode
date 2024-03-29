struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if s.len() <= num_rows as _ || num_rows == 1 {
            return s;
        }

        let mut matrix = Vec::<Vec<char>>::new();
        for _ in 0..num_rows {
            matrix.push(vec![]);
        }
        let rows_loopback = if s.len() == 2 {
            (0..=1).collect()
        } else {
            (0..num_rows)
                .chain((1..(num_rows - 1)).rev())
                .collect::<Vec<i32>>()
        };
        let rows_cycle = rows_loopback.iter().cycle();

        for (row, c) in rows_cycle.zip(s.chars().into_iter()) {
            matrix.get_mut(*row as usize).unwrap().push(c);
        }

        let mut flattened = vec![];
        for row in matrix.iter_mut() {
            flattened.append(row);
        }
        flattened.iter().collect::<String>()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 3),
            "PAHNAPLSIIGYIR".to_string()
        );
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 4),
            "PINALSIGYAHRPI".to_string()
        );
        assert_eq!(Solution::convert("A".to_string(), 1), "A".to_string());
        assert_eq!(Solution::convert("abcd".to_string(), 2), "acbd".to_string());
        assert_eq!(
            Solution::convert("ABCDEF".to_string(), 6),
            "ABCDEF".to_string()
        );
        assert_eq!(
            Solution::convert("ABCDEF".to_string(), 5),
            "ABCDFE".to_string()
        );
    }
}
