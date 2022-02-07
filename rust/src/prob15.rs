use std::collections::{HashMap, HashSet};

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
/*
pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    // speed up index_of_le by requiring a starting index
    fn index_of_le<T: std::cmp::PartialOrd>(
        v: Vec<T>,
        val: &T,
        index: usize,
    ) -> Option<(usize, &T)> {
        // this is safe because sorted_uniques is being passed to this
        // function, so the first value <= is the highest valid value

        v[..=index]
            .iter()
            .rev()
            .enumerate()
            .filter(|(i, &elem)| elem <= *val)
            .next()
    }
    // fn index_of_le<T: std::cmp::PartialOrd>(v: Vec<T>, val: T) -> Option<usize> {
    //     for (i, &elem) in v.iter().rev().enumerate() {
    //         // this is safe because sorted_uniques is being passed to this
    //         // function, so the first value <= is the highest valid value
    //         if elem <= val {
    //             return Some(i);
    //         }
    //     }
    //     None
    // }
    // There are no valid solutions in
    // this case so return empty vector
    if nums.len() < 3 {
        return vec![];
    }
    let mut sorted = nums.clone();
    sorted.sort_unstable();
    let mut bag = HashMap::with_capacity(sorted.len());
    // thought of alt algo where you do one insert per n val
    // counting them as you go through sortd then insert once
    let mut sorted_iter = sorted.iter();
    let mut current = *sorted_iter.next().unwrap();
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
    // if let Some(3) = bag.get(&0) {
    //     return vec![]
    // }
    // time complexity so far: n + nlogn

    let mut solutions = vec![];
    let mut lowest_valid_right_index;
    // this value decreases on each outer loop iteration
    let mut highest_valid_right_index; //= sorted_uniques.len() - 1;
    let mut right_val; // = *sorted_uniques.last().unwrap();

    // this lowers after every left increment
    // and every 3 sum found
    // let mut highest_valid_right_index = highest_valid_right_index.clone();

    if sorted_uniques.len() == 1 {
        if sorted_uniques[0] == 0 {
            return vec![vec![0, 0, 0]];
        } else {
            return vec![];
        }
    }
    // cut 2 off the end because space is needed for middle and right
    // actually only cut 1 off

    // left's bounds are from index 0 to the index of sorted_uniques.indexof(0)
    let mut left_iter = &sorted_uniques[..index_of_zero];
    'outer: for (left_index, &left_val) in left_iter.iter().enumerate() {
        // // special case
        // more efficient to short circuit to fail fast instead
        // of creating an array ref window each time
        // if left_val == 0 && &sorted[left_index + 1..left_index + 3] == &[0, 0] {
        //     solutions.push(vec![0, 0, 0]);
        //     break;
        // }
        // if left_val == 0 && sorted[left_index + 1] == 0 && sorted[left_index + 2] == 0 {
        // }
        // right_val = -(left_val * 2);
        let right_cannot_exceed = -(left_val * 2);
        highest_valid_right_index = index_of_le(
            sorted_uniques,
            right_cannot_exceed,
            highest_valid_right_index,
        )
        .unwrap();
        // highest_exst_valid_r_val = sorted[highest_valid_right_index];
        right_val = sorted[highest_valid_right_index];
        // let mut right = sorted.len() - 1;
        // let mut left_val = sorted[left];
        // let mut right_val = sorted[right];

        // right can never be greater than -(m*2)
        // advance right lefward when solution found
        // drop right

        let right_iter = &sorted_uniques[(left_index + 2)..=highest_valid_right_index];

        for (right_index, right_val) in right_iter.iter().enumerate() {
            // TODO: this needs to be inside inner for loop
            if right_index < left_index + 2 || left_val {
                break;
            }
        }

        let middle_opt = bag.get(&-(&left_val + &right_val));
        match middle_opt {
            Some(&middle) => {
                solutions.push(vec![left_val, middle, right_val]);
                right -= 1;
            }
            None => {
                // decrement right
                right -= 1;
            }
        }
    }
    if let Some(3) = bag.get(&0) {
        solutions.push(vec![0, 0, 0]);
    }
    solutions
}
*/
#[cfg(test)]
mod problem_tests {
    use super::*;

    #[test]
    fn test_three_sum1() {
        assert_eq!(
            three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![[-1, -1, 2], [-1, 0, 1]]
        );
    }
    #[test]
    fn test_three_sum2() {
        assert_eq!(three_sum(vec![]), Vec::<Vec<i32>>::new());
    }
    #[test]
    fn test_three_sum3() {
        assert_eq!(three_sum(vec![0]), Vec::<Vec<i32>>::new());
    }
    #[test]
    fn test_three_sum4() {
        assert_eq!(three_sum(vec![0, 0, 0]), vec![vec![0, 0, 0]]);
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
