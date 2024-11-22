// Time taken: 12:59, 13:03, 13:06 -> Acc
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
    pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut d1 = ListNode {
            val: -1,
            next: None,
        };
        let mut d2 = ListNode {
            val: -1,
            next: None,
        };

        let mut curr1 = &mut d1.next;
        let mut curr2 = &mut d2.next;

        while let Some(mut node) = head {
            head = node.next.take();

            if node.as_ref().val < x {
                curr1 = &mut curr1.insert(node).next;
            } else {
                curr2 = &mut curr2.insert(node).next;
            }
        }

        if let Some(node) = d2.next.take() {
            let _ = curr1.insert(node);
        }

        return d1.next;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
