// Time taken: 23:23, 13:03 -> Acc
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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode {
            val: -1,
            next: head,
        }));
        let mut slow: &mut Option<Box<ListNode>> = unsafe { &mut *(&mut dummy as *mut _) };
        let mut fast = &dummy;

        for _ in 0..=n {
            if let Some(node) = fast {
                fast = &node.next;
            }
        }

        while let Some(node) = fast {
            fast = &node.next;
            slow = &mut slow.as_mut().unwrap().next;
        }

        let next_node = slow.as_mut().unwrap().next.take();
        slow.as_mut().unwrap().next = next_node.unwrap().next;

        return dummy.unwrap().next;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
