// Time taken: 20:12, 20:17 -> Acc

use std::collections::HashSet;

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

struct Solution {}

impl Solution {
    pub fn num_components(mut head: Option<Box<ListNode>>, nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut arr = Vec::new();
        let mut set = HashSet::new();

        for num in nums {
            set.insert(num);
        }

        while let Some(node) = head {
            arr.push(node.val);
            head = node.next;
        }

        let mut present = false;
        for num in arr {
            if set.contains(&num) {
                if !present {
                    present = true;
                    result += 1;
                }
            } else {
                present = false;
            }
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
