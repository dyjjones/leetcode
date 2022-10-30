struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        match (nums1.len(), nums2.len()) {
            (0, 0) => {
                return 0.0;
            }
            (1, 0) => {
                return nums1[0] as _;
            }
            (0, 1) => {
                return nums2[0] as _;
            }
            _ => {}
        }
        let mut tree = std::collections::BTreeMap::new();
        for n in nums1.iter().chain(nums2.iter()) {
            match tree.get(n) {
                Some(&val) => {
                    tree.insert(n, val + 1);
                }
                None => {
                    tree.insert(n, 1);
                }
            }
        }
        let mut iter = tree.iter();

        let total = nums1.len() + nums2.len();
        let mut drop = if total % 2 == 0 {
            total / 2 - 1
        } else {
            total / 2
        };
        let mut remainder = (0, 0);
        for (&k, v) in &mut iter {
            if drop < *v {
                remainder = (*k, v - drop);
                break;
            } else if drop == *v {
                remainder = (*k, 0);
                // return *k as _;
                break;
            } else {
                drop -= v;
            }
        }
        match (remainder, total % 2 == 0) {
            ((_, 0), true) => {
                let next1 = iter.next().unwrap();
                if *next1.1 == 1 as usize {
                    let next2 = iter.next().unwrap();
                    return (*next1.0 + *next2.0) as f64 / 2.0;
                } else {
                    return **next1.0 as _;
                }
            }
            ((_, 0), false) => {
                return **iter.next().unwrap().0 as _;
            }
            ((k, 1), true) => {
                return (k + **iter.next().unwrap().0) as f64 / 2.0;
            }
            ((k, 1), false) => {
                return k as _;
            }
            ((k, _), _) => {
                return k as _;
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_median_sorted_arrays() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2.0
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        );
        assert_eq!(Solution::find_median_sorted_arrays(vec![], vec![]), 0.0);
        assert_eq!(Solution::find_median_sorted_arrays(vec![1], vec![]), 1.0);
        assert_eq!(Solution::find_median_sorted_arrays(vec![1], vec![1]), 1.0);
        assert_eq!(Solution::find_median_sorted_arrays(vec![5], vec![1]), 3.0);
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![5, 5], vec![1]),
            5.0
        );
        assert_eq!(Solution::find_median_sorted_arrays(vec![1, 1], vec![]), 1.0);
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![], vec![1, 2, 3, 3, 3, 3]),
            3.0
        );
    }
}
