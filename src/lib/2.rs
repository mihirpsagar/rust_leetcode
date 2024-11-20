// Time taken: 16:55, 17:09, 17:24 -> Acc
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
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut curr_node = &mut head;
        let mut remainder = 0;

        while l1.is_some() || l2.is_some() || remainder > 0 {
            let sum =
                l1.as_ref().map_or(0, |x| x.val) + l2.as_ref().map_or(0, |x| x.val) + remainder;
            remainder = sum / 10;

            let new_node = curr_node.insert(Box::new(ListNode::new(sum % 10)));
            curr_node = &mut new_node.next;

            l1 = l1.and_then(|x| x.next);
            l2 = l2.and_then(|x| x.next);
        }

        return head;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
