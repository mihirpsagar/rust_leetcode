// Time taken: 23:00, 23:11 -> Acc

use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
};

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
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(-1);
        let mut min_heap = BinaryHeap::new();

        for mut list in lists {
            while let Some(node) = list {
                min_heap.push(Reverse(node.val));
                list = node.next;
            }
        }

        let mut curr = &mut dummy.next;
        while let Some(val) = min_heap.pop() {
            let new_node = curr.insert(Box::new(ListNode::new(val.0)));
            curr = &mut new_node.next;
        }

        return dummy.next;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
