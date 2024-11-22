// Time taken: 19:45, 20:15 -> Wrong, 20:20 -> Acc
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
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }

        let mut dummy = ListNode {
            val: -1,
            next: head,
        };
        let mut dummy2 = ListNode {
            val: -1,
            next: None,
        };
        let mut curr = &dummy.next;
        let mut len = 0;
        while let Some(node) = curr {
            len += 1;
            curr = &node.next;
        }

        let k = k % len;

        if k == 0 {
            return dummy.next;
        }

        let mut left = &mut dummy;
        for _ in 0..(len - k) {
            left = left.next.as_mut().unwrap();
        }

        dummy2.next = left.next.take();
        // let mut mid: &mut ListNode = unsafe { &mut *(&mut dummy2 as *mut _) };
        let mut mid = &mut dummy2;
        while mid.next.is_some() {
            mid = mid.next.as_mut().unwrap();
        }

        mid.next = dummy.next.take();
        dummy.next = dummy2.next.take();

        return dummy.next;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
