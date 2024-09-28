// Time taken: 23:55, 00:03 -> Acc
struct Solution {}

impl Solution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        let mut largest: (Option<(i32, usize)>, Option<i32>) = (None, None);

        for (idx, &num) in nums.iter().enumerate() {
            if let Some(l1) = largest.0 {
                if let Some(l2) = largest.1 {
                    if num > l1.0 {
                        largest.1 = Some(l1.0);
                        largest.0 = Some((num, idx));
                    } else if num > l2 {
                        largest.1 = Some(num);
                    }
                } else {
                    if num > l1.0 {
                        largest.0 = Some((num, idx));
                        largest.1 = Some(l1.0);
                    } else {
                        largest.1 = Some(num);
                    }
                }
            } else {
                largest.0 = Some((num, idx));
            }
        }

        if (largest.0.unwrap().0) >= (2 * largest.1.unwrap()) {
            return largest.0.unwrap().1 as i32;
        } else {
            return -1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::dominant_index(vec![3, 6, 1, 0]), 1);
        assert_eq!(Solution::dominant_index(vec![1, 2, 3, 4]), -1);
    }
}
