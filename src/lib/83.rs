// Time taken: 22:51, 23:02 -> Wrong, 23:31 -> Optimized

use std::ops::DerefMut;

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
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut pointer = &mut head;

        while let Some(node1) = pointer {
            while let Some(node2) = &mut node1.next {
                if node1.val == node2.val {
                    node1.next = node2.next.take();
                } else {
                    break;
                }
            }
            pointer = &mut node1.next;
        }

        return head;
    }

    // pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    //     let mut p1 = head.as_mut();

    //     while let Some(node1) = p1 {
    //         let mut p2 = node1.next.take();

    //         while let Some(node2) = p2.as_mut() {
    //             if node1.val == node2.val {
    //                 p2 = node2.next.take();
    //             } else {
    //                 node1.next = p2;
    //                 break;
    //             }
    //         }

    //         p1 = node1.next.as_mut();
    //     }

    //     return head;
    // }

    // pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    //     let mut pointer = &mut head;

    //     loop {
    //         if let Some(node1) = pointer.as_mut() {
    //             if let Some(node2) = node1.next.as_mut() {
    //                 if node1.val == node2.val {
    //                     node1.next = node2.next.take();
    //                     pointer = &mut Some(node1);
    //                 } else {
    //                     pointer = &mut node1.next;
    //                 }
    //             } else {
    //                 break;
    //             }
    //         } else {
    //             break;
    //         }
    //     }

    //     return head;
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
