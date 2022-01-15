use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // First, create multiset/bag of nums to reduce time complexity
    // from n^2 to n (average)
    // let set: HashSet<_> = nums.iter().cloned().collect();

    // in this case we only care that there is:
    // * 1
    // * > 1 instances of a number. this case is for target - n = n
    let mut bag: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
    for (index, n) in nums.iter().enumerate() {
        let searchfor = target - n;
        match bag.get(&searchfor) {
            Some(result) => {
                return vec![*result as _, index as _];
            }
            None => {
                bag.insert(*n, index);
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod problem_tests {
    use super::*;

    #[test]
    fn test_is_match() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
