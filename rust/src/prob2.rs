// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    // commented out to satisfy LeetCode
    // It doesn't accept the below modifications to impl ListNode

    // fn prepend(self, val: i32) -> Self {
    //     ListNode {
    //         next: Some(Box::new(self)),
    //         val,
    //     }
    // }
    // fn build(values: Vec<i32>) -> Option<Box<Self>> {
    //     if values.len() == 0 {
    //         return None;
    //     }
    //     let mut values_iter = values.iter().rev();
    //     let mut list = ListNode::new(*values_iter.next().unwrap());
    //     for &v in values_iter {
    //         list = list.prepend(v);
    //     }
    //     Some(Box::new(list))
    // }
}

fn prepend(l: ListNode, val: i32) -> ListNode {
    ListNode {
        next: Some(Box::new(l)),
        val,
    }
}
fn build(values: Vec<i32>) -> Option<Box<ListNode>> {
    if values.is_empty() {
        return None;
    }
    let mut values_iter = values.iter().rev();
    let mut list = ListNode::new(*values_iter.next().unwrap());
    for &v in values_iter {
        list = prepend(list, v);
    }
    Some(Box::new(list))
}

// incomplete attempt to make ListNode iterable

// pub struct ListNodeIter {
//     current_node: Option<Box<ListNode>>,
// }

// impl Iterator for ListNodeIter {
//     type Item = i32;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.current_node = match &self.current_node {
//             None => {
//                 return None;
//             }
//             Some(ln) => {
//                 // advance current node to next
//                 let n1 = (*ln);
//                 let next_node = n1.next;
//                 // return new current node's value
//                 match next_node {
//                     Some(_) => next_node,
//                     None => {
//                         return None;
//                     }
//                 }
//             }
//         };
//         Some(self.current_node.unwrap().val)
//     }
// }

// impl IntoIterator for ListNode {
//     type Item = i32;
//     type IntoIter = ListNodeIter;
//     // type IntoIter = std::vec::IntoIter<Self::Item>;
//     // type IntoIter: Iterator;
//     fn into_iter(self) -> Self::IntoIter {
//         ListNodeIter {
//             current_node: Some(Box::new(self)),
//         }
//     }
// }

// pub fn add_two_numbers(
//     l1: Option<Box<ListNode>>,
//     l2: Option<Box<ListNode>>,
// ) -> Option<Box<ListNode>> {
//     let mut zip_iter = l1.unwrap().into_iter().zip(l2.unwrap().into_iter());
//     let (first_l1, first_l2) = zip_iter.next().unwrap();

//     let mut carry = false;
//     let mut val = first_l1 + first_l2;

//     if 9 < val {
//         carry = true;
//         val -= 10;
//     }
//     let current_node = ListNode {
//         val: val,
//         next: None,
//     };
//     let mut summation: Option<Box<ListNode>> = Some(Box::new(current_node));

//     for (ln1, ln2) in zip_iter {
//         println!("test {} {}", ln1, ln2);
//         val = ln1 + ln2;
//         if carry {
//             val += 1;
//         }
//         if 9 < val {
//             val -= 10;
//             carry = true;
//         } else {
//             carry = false;
//         }
//         let current_node = ListNode {
//             val: val,
//             next: summation,
//         };
//         summation = Some(Box::new(current_node));
//     }
//     if carry {
//         summation = Some(Box::new(ListNode {
//             val: 1,
//             next: summation,
//         }));
//     }

//     summation
// }

struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut carry = false;
        let mut current_l1 = &l1;
        let mut current_l2 = &l2;

        let mut digits = vec![];
        let mut val1: i32;
        let mut val2: i32;
        loop {
            let mut iter_sum = (carry as i32)
                + match (current_l1, current_l2) {
                    (Some(first), Some(second)) => {
                        current_l1 = &first.next;
                        current_l2 = &second.next;
                        val1 = first.val;
                        val2 = second.val;

                        val1 + val2
                    }
                    (None, None) => break,
                    (Some(only), None) | (None, Some(only)) => {
                        current_l1 = &only.next;
                        current_l2 = &None;
                        only.val
                    }
                };
            if iter_sum < 10 {
                carry = false;
            } else {
                carry = true;
                iter_sum -= 10;
            }
            digits.push(iter_sum);
        }
        if carry {
            digits.push(1);
        }
        println!("digits: {digits:?}");
        build(digits)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two_numbers1() {
        let l1 = build(vec![2, 4, 3]);
        let l2 = build(vec![5, 6, 4]);
        let expected = build(vec![7, 0, 8]);
        assert_eq!(Solution::add_two_numbers(l1, l2), expected);
    }

    #[test]
    fn test_add_two_numbers2() {
        let l1 = build(vec![9, 9, 9, 9, 9, 9, 9]);
        let l2 = build(vec![9, 9, 9, 9]);
        let expected = build(vec![8, 9, 9, 9, 0, 0, 0, 1]);
        assert_eq!(Solution::add_two_numbers(l1, l2), expected);
    }
}
