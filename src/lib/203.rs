// Time taken: 13:36, 13:45 -> Wrong, 13:56 -> Acc
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
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut dummy = ListNode {
            next: head,
            val: -1,
        };
        let mut curr_node = &mut dummy;

        while let Some(node) = curr_node.next.as_mut() {
            if node.val == val {
                curr_node.next = node.next.take();
            } else {
                curr_node = curr_node.next.as_mut().unwrap();
            }
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
