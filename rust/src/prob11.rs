// fn advance_by<'a, I>(mut it: I, n: &i32) -> I
// where
//     I: Iterator<Item = (usize, &'a i32)>,
// {
//     for _ in 0..*n {
//         it.next();
//     }
//     it
// }

// struct Max {
//     index: usize,
//     val: i32,
// }

// #[derive(PartialEq, Eq, Debug)]
// enum Side {
//     Left,
//     Right,
// }
// impl Side {
//     fn flip(&mut self) {
//         *self = match self {
//             Self::Left => Self::Right,
//             Self::Right => Self::Left,
//         }
//     }
// }

struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut left = 0_usize;
        let mut right = height.len() - 1;

        // outside loop for first vals
        let mut height_left = height[left];
        let mut height_right = height[right];
        let mut current_area = (right - left) as i32 * height_left.min(height_right);
        if max < current_area {
            max = current_area;
        }

        loop {
            // move shorter one over
            if height_left < height_right {
                // move left to the right
                left += 1;
            } else {
                // move right to the left
                right -= 1;
            }

            if right <= left {
                break;
            }

            height_left = height[left];
            height_right = height[right];
            current_area = (right - left) as i32 * height_left.min(height_right);
            if max < current_area {
                max = current_area;
            }
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_area() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
    }
}
