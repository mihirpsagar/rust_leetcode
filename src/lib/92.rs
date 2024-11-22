// Time taken: 13:09, 13:43, 13:53 -> Acc
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
    pub fn reverse_between(
        mut head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        let mut dummy1 = None;
        let mut dummy2 = None;
        let mut curr1 = &mut dummy1;
        let mut curr2 = &mut dummy2;
        let mut count = 0;

        while let Some(mut node) = head {
            count += 1;
            head = node.next.take();
            if count < left {
                curr1 = &mut curr1.insert(node).next;
            } else if count < right {
                curr2 = &mut curr2.insert(node).next;
            } else if count == right {
                curr2 = &mut curr2.insert(node).next;
                break;
            }
        }

        let mut rev_head = None;
        while let Some(mut node) = dummy2 {
            let next = node.next.take();
            node.next = rev_head;
            rev_head = Some(node);
            dummy2 = next;
        }

        if let Some(node) = rev_head {
            curr1 = &mut curr1.insert(node).next;
        }

        while curr1.is_some() {
            curr1 = &mut curr1.as_mut()?.next;
        }

        while let Some(mut node) = head {
            head = node.next.take();
            curr1 = &mut curr1.insert(node).next;
        }

        return dummy1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
