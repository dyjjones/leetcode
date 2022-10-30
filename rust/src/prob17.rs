use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let nums_to_chars_vec = vec![
            ('2', vec!['a', 'b', 'c']),
            ('3', vec!['d', 'e', 'f']),
            ('4', vec!['g', 'h', 'i']),
            ('5', vec!['j', 'k', 'l']),
            ('6', vec!['m', 'n', 'o']),
            ('7', vec!['p', 'q', 'r', 's']),
            ('8', vec!['t', 'u', 'v']),
            ('9', vec!['w', 'x', 'y', 'z']),
        ];
        let mut nums_to_chars = HashMap::<char, Vec<char>>::new();
        for (numc, charsv) in nums_to_chars_vec {
            nums_to_chars.insert(numc, charsv);
        }

        let letter_product = digits
            .chars()
            .map(|c| nums_to_chars.get(&c).unwrap())
            .collect::<Vec<_>>();

        product(&letter_product)
            .into_iter()
            .map(|v| v.iter().collect::<String>())
            .collect::<Vec<_>>()
    }
}

fn product(matrix: &Vec<&Vec<char>>) -> Vec<Vec<char>> {
    let mut res = vec![];

    let mut matrix_iter = matrix.iter();
    if let Some(&first_vec) = matrix_iter.next() {
        for &i in first_vec.iter() {
            res.push(vec![i]);
        }
    }

    for &row in matrix_iter {
        let mut temp = vec![];
        for r in res {
            for &el in row {
                let mut temp_el = r.clone();
                temp_el.push(el);
                temp.push(temp_el);
            }
        }

        res = temp;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_letter_combinations1() {
        assert_eq!(
            Solution::letter_combinations(String::from("23")),
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
                .iter()
                .map(|&s| String::from(s))
                .collect::<Vec<String>>()
        );
    }
    #[test]
    fn test_letter_combinations2() {
        assert_eq!(
            Solution::letter_combinations(String::from("")),
            Vec::<String>::new()
        );
    }
    #[test]
    fn test_letter_combinations3() {
        assert_eq!(
            Solution::letter_combinations(String::from("2")),
            vec!["a", "b", "c"]
                .iter()
                .map(|&s| String::from(s))
                .collect::<Vec<String>>()
        );
    }

    #[test]
    fn test_letter_combinations4() {
        assert_eq!(
            Solution::letter_combinations(String::from("5467")),
            vec![
                "jgmp", "jgmq", "jgmr", "jgms", "jgnp", "jgnq", "jgnr", "jgns", "jgop", "jgoq",
                "jgor", "jgos", "jhmp", "jhmq", "jhmr", "jhms", "jhnp", "jhnq", "jhnr", "jhns",
                "jhop", "jhoq", "jhor", "jhos", "jimp", "jimq", "jimr", "jims", "jinp", "jinq",
                "jinr", "jins", "jiop", "jioq", "jior", "jios", "kgmp", "kgmq", "kgmr", "kgms",
                "kgnp", "kgnq", "kgnr", "kgns", "kgop", "kgoq", "kgor", "kgos", "khmp", "khmq",
                "khmr", "khms", "khnp", "khnq", "khnr", "khns", "khop", "khoq", "khor", "khos",
                "kimp", "kimq", "kimr", "kims", "kinp", "kinq", "kinr", "kins", "kiop", "kioq",
                "kior", "kios", "lgmp", "lgmq", "lgmr", "lgms", "lgnp", "lgnq", "lgnr", "lgns",
                "lgop", "lgoq", "lgor", "lgos", "lhmp", "lhmq", "lhmr", "lhms", "lhnp", "lhnq",
                "lhnr", "lhns", "lhop", "lhoq", "lhor", "lhos", "limp", "limq", "limr", "lims",
                "linp", "linq", "linr", "lins", "liop", "lioq", "lior", "lios"
            ]
            .iter()
            .map(|&s| String::from(s))
            .collect::<Vec<String>>()
        );
    }
}
