// Time taken: 22:48, 23:06 -> Acc

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
    pub fn insertion_sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result = None;

        while let Some(mut node) = head.take() {
            let next = node.next.take();
            result = Self::insert(result, node);
            head = next;
        }

        return result;
    }

    pub fn insert(node1: Option<Box<ListNode>>, mut node2: Box<ListNode>) -> Option<Box<ListNode>> {
        if let Some(mut inner_node1) = node1 {
            if node2.val < inner_node1.val {
                node2.next = Some(inner_node1);
                return Some(node2);
            } else {
                let next = inner_node1.next.take();
                inner_node1.next = Self::insert(next, node2);
                return Some(inner_node1);
            }
        } else {
            return Some(node2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
