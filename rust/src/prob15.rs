use std::collections::{HashMap, HashSet};

struct Solution;

impl Solution {
    // this naive solution works, at around 180 ms on leetcode
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        // algo:
        // first sort
        //

        // speed up index_of_le by requiring a starting index
        /*    fn index_of_le<T: std::cmp::PartialOrd>(
            v: Vec<T>,
            val: &T,
            index: usize,
        ) -> Option<(usize, &T)> {
            // let reversed_iter = v.iter().rev();
            // let to_drop = v.len() - index - 1; // -1 is for conversion from len to last index
            // reversed_iter.advance_by(to_drop);
            // let first_match =
            // this is safe because sorted_uniques is being passed to this
            // function, so the first value <= is the highest valid value

            &(v[..=index])
                .iter()
                .rev()
                .enumerate()
                .filter(|(_, &elem)| elem <= *val)
                .next()
            // match first_match {
            //     Some((i, elem)) => {}
            // }
            // for (i, &elem) in v.iter().rev().enumerate() {
            //     // this is safe because sorted_uniques is being passed to this
            //     // function, so the first value <= is the highest valid value
            //     if elem <= val {
            //         return Some(i);
            //     }
            // }
            // None
        }
        */
        if nums.len() < 3 {
            return vec![];
        }

        // let mut sorted = nums.clone();
        // sorted.sort_unstable();
        let mut nums_iter = nums.iter();
        let mut bag = HashMap::with_capacity(nums.len());
        let first = *nums_iter.next().unwrap();
        let second = *nums_iter.next().unwrap();
        let mut solutions = vec![];
        let mut solutions_set = HashSet::<(i32, i32, i32)>::new();
        if first == second {
            bag.insert(second, 2);
        } else {
            bag.insert(first, 1);
            bag.insert(second, 1);
        }
        for &right in nums_iter {
            for &left in bag.keys() {
                let middle = -(right + left);
                let mut solution: Vec<i32> = vec![left, middle, right];
                solution.sort();
                if let Some(&val) = bag.get(&middle) {
                    if !solutions_set.contains(&(solution[0], solution[1], solution[2]))
                        && (left != middle || 1 < val)
                    {
                        solutions_set.insert((solution[0], solution[1], solution[2]));
                        solutions.push(solution);
                    }
                }
            }
            bag.insert(
                right,
                match bag.get(&right) {
                    Some(val) => val + 1,
                    None => 1,
                },
            );
        }
        solutions.sort();
        solutions
        // thought of alt algo where you do one insert per n val
        // counting them as you go through sortd then insert once
        // let mut sorted_iter = sorted.iter();
        // let mut current = *sorted_iter.next().unwrap();
        /*
        let mut count = 1;

        let index_of_zero;
        let mut sorted_uniques = vec![];
        // this inserts once per n value
        // more efficient than multiple inserts per n
        // i.e. insert for each elem in the sorted vector
        for &n in sorted_iter {
            if n == 0 {
                index_of_zero = sorted_uniques.len();
            }
            if current == n {
                count += 1;
            } else {
                bag.insert(current, count);
                sorted_uniques.push(current);
                current = n;
                count = 1;
            }
        }
        // insert final element
        bag.insert(current, count);
        sorted_uniques.push(current);
        */

        // primitive algo:
        // unsorte, as you interate your "left" val through the array once
        // "middle" and "right" are taken from the values you have already seen
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_sum1() {
        assert_eq!(
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![[-1, -1, 2], [-1, 0, 1]]
        );
    }
    #[test]
    fn test_three_sum2() {
        assert_eq!(Solution::three_sum(vec![]), Vec::<Vec<i32>>::new());
    }
    #[test]
    fn test_three_sum3() {
        assert_eq!(Solution::three_sum(vec![0]), Vec::<Vec<i32>>::new());
    }
    #[test]
    fn test_three_sum4() {
        assert_eq!(Solution::three_sum(vec![0, 0, 0]), vec![vec![0, 0, 0]]);
        // -307 -306 613
        // right upper bound = min(614, sorted.last())
        // right lower bound = half, +1 if left is odd, 154
        // so window for right is 154 to 614
        // -613 306 307
        // right upper bound = min(1226, sorted.last())
        // right lower bound = 307
        // window for right is 307,
    }
}
