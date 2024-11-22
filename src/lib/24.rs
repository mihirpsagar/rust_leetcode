// Time taken: 17:55, 18:31 -> Acc
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
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode {
            val: -1,
            next: head,
        };

        let mut left = &mut dummy;

        while let Some(mut mid) = left.next.take() {
            if let Some(mut right) = mid.next.take() {
                mid.next = right.next;
                right.next = Some(mid);
                left.next = Some(right);
                left = left.next.as_mut().unwrap().next.as_mut().unwrap();
            } else {
                left.next = Some(mid);
                break;
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
