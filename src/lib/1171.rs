// Time taken: 20:32, 21:02 -> Acc

struct Solution {}

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
}

impl Solution {
    pub fn remove_zero_sum_sublists(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut arr = Vec::new();

        while let Some(node) = head {
            if (node.val != 0) {
                arr.push(node.val);
            }
            head = node.next;
        }

        if arr.is_empty() {
            return None;
        }

        let mut result = None;
        let mut curr = &mut result;
        let mut result_arr = vec![true; arr.len()];

        let mut left = 0;
        while left < arr.len() {
            let mut right = arr.len() - 1;
            let mut sum = 0;
            for k in left..=right {
                sum += arr[k];
            }

            while sum != 0 && left < right {
                sum -= arr[right];
                right -= 1;
            }

            if sum == 0 {
                for k in left..=right {
                    result_arr[k] = false;
                }
                left = right + 1;
            } else {
                left += 1;
            }
        }

        let mut idx = 0;
        while idx < arr.len() {
            if result_arr[idx] {
                curr = &mut curr.insert(Box::new(ListNode::new(arr[idx]))).next;
            }
            idx += 1;
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
