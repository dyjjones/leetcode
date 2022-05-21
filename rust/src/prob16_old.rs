pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    let mut lowest_delta = i32::MAX;
    let mut sums_set = HashSet::<i32>::new();
    // abs(target - sum) => sum
    let mut delta_to_sum = HashMap::<i32, i32>::new();

    // bag of values from nums
    let mut bag = HashMap::with_capacity(nums.len());

    let mut nums = nums;
    nums.sort();
    let mut nums_iter = nums.iter();

    let first = *nums_iter.next().unwrap();
    let second = *nums_iter.next().unwrap();
    if first == second {
        bag.insert(second, 2);
    } else {
        bag.insert(first, 1);
        bag.insert(second, 1);
    }
    let first_3 = bag.keys().collect::<Vec<_>>();
    let mut lowest_bag = **first_3.iter().min().unwrap();
    let mut highest_bag = **first_3.iter().max().unwrap();

    for &right in nums_iter {
        let mut sorted_bag_keys = bag.keys().collect::<Vec<_>>();
        sorted_bag_keys.sort();
        for &left in sorted_bag_keys {
            // middle in bag means an exact match and delta of 0
            let middle = target - (right + left);
            if let Some(&val) = bag.get(&middle) {
                if left != middle || 1 < val {
                    return target;
                }
            } else {
                // no exact match was found
                // so iterate from 1 to & excluding lowest delta
                'delta: for delta in 1..lowest_delta {
                    let middle_low = middle - delta;
                    let middle_high = middle + delta;
                    if middle_low < lowest_bag && highest_bag < middle_high {
                        break;
                    }
                    for possible_middle in [middle_low, middle_high] {
                        if let Some(&val) = bag.get(&possible_middle) {
                            if left != possible_middle || 1 < val {
                                lowest_delta = delta;
                                let sum = left + possible_middle + right;
                                delta_to_sum.insert(delta, sum);
                                if !sums_set.contains(&sum) {
                                    sums_set.insert(sum);
                                }
                                break 'delta;
                            }
                        }
                    }
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
        if right < lowest_bag {
            lowest_bag = right;
        } else if highest_bag < right {
            highest_bag = right;
        }
    }
    *delta_to_sum.get(&lowest_delta).unwrap()
}
