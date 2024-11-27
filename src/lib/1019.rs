// Time taken: 20:19, 20:24, 20:30 -> Acc

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
    pub fn next_larger_nodes(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut arr = Vec::new();
        let mut stack = Vec::new();

        while let Some(node) = head {
            arr.push(node.val);
            head = node.next;
        }

        let mut result = vec![0; arr.len()];

        for (idx, num) in arr.iter().enumerate() {
            loop {
                if !stack.is_empty() && arr[stack[stack.len() - 1]] < *num {
                    result[stack[stack.len() - 1]] = *num;
                    stack.pop();
                } else {
                    break;
                }
            }

            stack.push(idx);
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
