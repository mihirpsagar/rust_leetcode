// Time taken: 20:36, 20:40, 12:58 -> Acc
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
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode {
            val: -1,
            next: None,
        };

        let mut curr = &mut dummy.next;

        while let Some(mut node) = head {
            if node.next.is_none() || node.next.as_ref()?.val != node.val {
                head = node.next.take();
                curr = &mut curr.insert(node).next;
            } else {
                head = node.next?.next.take();
                while head.is_some() && head.as_ref()?.val == node.val {
                    head = head.as_mut()?.next.take();
                }
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
