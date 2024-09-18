// Time taken: 14:33, 14:54 -> Acc
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
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut result = true;

        if head.as_ref().is_none() || head.as_ref().unwrap().next.is_none() {
            return result;
        }

        let mut slow = head.as_ref();
        let mut fast = head.as_ref().unwrap().next.as_ref();

        while fast.as_ref().is_some() && fast.as_ref().unwrap().next.as_ref().is_some() {
            slow = slow.unwrap().next.as_ref();
            fast = fast.unwrap().next.as_ref().unwrap().next.as_ref();
        }

        let mut second_half = vec![];
        slow = slow.unwrap().next.as_ref();
        while let Some(node) = slow {
            second_half.push(node.val);
            slow = slow.unwrap().next.as_ref();
        }

        let mut curr = head.as_ref();
        while let Some(val) = second_half.pop() {
            if val != curr.unwrap().val {
                result = false;
                break;
            }
            curr = curr.unwrap().next.as_ref();
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
