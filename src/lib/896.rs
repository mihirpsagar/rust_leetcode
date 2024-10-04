// Time taken: 22:59, 23:05 -> Acc
struct Solution {}

impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        #[derive(PartialEq)]
        enum Order {
            Increasing,
            Decreasing,
        }
        let mut order = None;
        let mut idx = 1;

        while idx < nums.len() {
            match nums[idx - 1].cmp(&nums[idx]) {
                std::cmp::Ordering::Equal => {}
                std::cmp::Ordering::Greater => {
                    if let Some(val) = &order {
                        if *val == Order::Increasing {
                            return false;
                        }
                    } else {
                        order = Some(Order::Decreasing);
                    }
                }
                std::cmp::Ordering::Less => {
                    if let Some(val) = &order {
                        if *val == Order::Decreasing {
                            return false;
                        }
                    } else {
                        order = Some(Order::Increasing);
                    }
                }
            }
            idx += 1;
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::is_monotonic(vec![1, 2, 2, 3]), true);
        assert_eq!(Solution::is_monotonic(vec![6, 5, 4, 4]), true);
        assert_eq!(Solution::is_monotonic(vec![1, 3, 2]), false);
    }
}
