use std::collections::VecDeque;

// Time taken: 09:42, 09:47 -> Acc, 09:53 -> Optimized
struct Solution {}

impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let mut result = 0;
        let k = k as usize;
        let mut idx = 0;

        while idx < tickets.len() {
            if idx <= k {
                result += std::cmp::min(tickets[idx], tickets[k]);
            } else {
                result += std::cmp::min(tickets[idx], tickets[k] - 1);
            }
            idx += 1;
        }

        return result;
    }

    // pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
    //     let mut queue = VecDeque::new();
    //     let k = k as usize;
    //     let mut result = 0;

    //     for (idx, &num) in tickets.iter().enumerate() {
    //         queue.push_back((idx, num));
    //     }

    //     while let Some(mut node) = queue.pop_front() {
    //         node.1 -= 1;
    //         result += 1;
    //         if node.0 == k && node.1 == 0 {
    //             break;
    //         }
    //         if node.1 != 0 {
    //             queue.push_back(node);
    //         }
    //     }

    //     return result;
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::time_required_to_buy(vec![2, 3, 2], 2), 6);
        assert_eq!(Solution::time_required_to_buy(vec![5, 1, 1, 1], 0), 8);
    }
}
