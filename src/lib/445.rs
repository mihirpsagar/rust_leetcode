// Time taken: 14:05, 14:20 -> Acc

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
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut num1 = Vec::new();
        let mut num2 = Vec::new();
        let mut num3 = Vec::new();
        let mut result: Option<Box<ListNode>> = None;
        let mut result_curr = &mut result;

        while let Some(mut node) = l1 {
            num1.push(node.val);
            l1 = node.next.take();
        }

        while let Some(mut node) = l2 {
            num2.push(node.val);
            l2 = node.next.take();
        }

        let mut carry = 0;
        let mut idx1 = num1.len() - 1;
        let mut idx2 = num2.len() - 1;

        loop {
            let sum = num1[idx1] + num2[idx2] + carry;
            carry = sum / 10;
            num3.push(sum % 10);
            if idx1 == 0 || idx2 == 0 {
                break;
            }
            idx1 -= 1;
            idx2 -= 1;
        }

        loop {
            if idx1 == 0 {
                break;
            }
            idx1 -= 1;
            let sum = num1[idx1] + carry;
            carry = sum / 10;
            num3.push(sum % 10);
        }

        loop {
            if idx2 == 0 {
                break;
            }
            idx2 -= 1;
            let sum = num2[idx2] + carry;
            carry = sum / 10;
            num3.push(sum % 10);
        }

        if carry == 1 {
            num3.push(carry);
        }

        let mut idx3 = num3.len() - 1;
        loop {
            let new_node = ListNode::new(num3[idx3]);
            result_curr = &mut result_curr.insert(Box::new(new_node)).next;
            if idx3 == 0 {
                break;
            }
            idx3 -= 1;
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
