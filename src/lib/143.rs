// Time taken: 21:56, 22:02, 22:09 -> Acc
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
    pub fn reorder_list(mut head: &mut Option<Box<ListNode>>) {
        let mut arr = Vec::new();
        let mut result = None;
        let mut result_curr = &mut result;
        let mut head_clone = head.clone();
        while let Some(mut node) = head_clone {
            arr.push(node.val);
            head_clone = node.next.take();
        }

        let mut left = 0;
        let mut right = arr.len() - 1;
        while left < right {
            result_curr = &mut result_curr.insert(Box::new(ListNode::new(arr[left]))).next;
            result_curr = &mut result_curr.insert(Box::new(ListNode::new(arr[right]))).next;
            left += 1;
            right -= 1;
        }

        if left == right {
            result_curr = &mut result_curr.insert(Box::new(ListNode::new(arr[left]))).next;
        }

        *head = result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
