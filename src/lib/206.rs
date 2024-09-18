// Time taken: 21:53, 22:01 -> Wrong, 22:15 -> Acc
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
    /// Pseudo code:
    /// while(curr != None) {
    ///     next = curr->next;
    ///     curr->next = prev;
    ///     prev = curr;
    ///     curr = next;
    /// }
    /// return prev;
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = None;
        let mut curr = head;

        while let Some(mut curr_node) = curr {
            let next = curr_node.next.take();
            curr_node.next = dummy.take();
            dummy = Some(curr_node);
            curr = next;
        }
        return dummy;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
