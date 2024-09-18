// Time taken: 22:23, 23:09, 00:24 -> Optimized code

use std::{cmp::Ordering, rc::Rc};

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
    // fn new(val: i32, next: Option<Box<ListNode>>) -> Self {
    //     ListNode { next: next, val }
    // }
}

impl Solution {
    pub fn merge_two_lists(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy_node = ListNode::new(0);
        let mut curr_node = &mut dummy_node;

        while let (Some(node1), Some(node2)) = (l1.as_ref(), l2.as_ref()) {
            if node1.val <= node2.val {
                curr_node.next = l1.take();
                l1 = curr_node.next.as_mut().unwrap().next.take();
            } else {
                curr_node.next = l2.take();
                l2 = curr_node.next.as_mut().unwrap().next.take();
            }
            curr_node = curr_node.next.as_mut().unwrap();
        }

        curr_node.next = l1.or(l2);

        return dummy_node.next;
    }

    // pub fn merge_two_lists(
    //     mut l1: Option<Box<ListNode>>,
    //     mut l2: Option<Box<ListNode>>,
    // ) -> Option<Box<ListNode>> {
    //     let mut r = &mut l1;
    //     while l2.is_some() {
    //         println!("R Before: {:?}", &r);
    //         println!("L2 Before: {:?}", &l2);
    //         if r.is_none() || l2.as_ref()?.val < r.as_ref()?.val {
    //             std::mem::swap(r, &mut l2);
    //             println!("R Middle: {:?}", &r);
    //             println!("L2 Middle: {:?}", &l2);
    //         }
    //         r = &mut r.as_mut()?.next;
    //         println!("R After: {:?}", &r);
    //         println!("L2 After: {:?}", &l2);
    //     }
    //     println!("L1 After: {:?}", &l1);
    //     l1
    // }

    // pub fn merge_two_lists(
    //     mut list1: Option<Box<ListNode>>,
    //     mut list2: Option<Box<ListNode>>,
    // ) -> Option<Box<ListNode>> {
    //     if list1 == None && list2 == None {
    //         return None;
    //     }

    //     let mut result: Option<Rc<Box<ListNode>>>;
    //     let mut root: Option<Rc<Box<ListNode>>>;

    //     loop {
    //         let mut v1 = None;
    //         let mut v2 = None;
    //         let mut new_node = Rc::new(Box::new(ListNode::new(0)));
    //         match list1 {
    //             None => {}
    //             Some(ref node1) => {
    //                 v1 = Some(node1.val);
    //             }
    //         }
    //         match list2 {
    //             None => {}
    //             Some(ref node2) => {
    //                 v2 = Some(node2.val);
    //             }
    //         }

    //         if v1 != None {
    //             if v2 != None {
    //                 match v1.cmp(&v2) {
    //                     Ordering::Less | Ordering::Equal => {
    //                         new_node.val = v1.unwrap();
    //                         list1 = list1.unwrap().next;
    //                     }
    //                     Ordering::Greater => {
    //                         new_node.val = v2.unwrap();
    //                         list2 = list2.unwrap().next;
    //                     }
    //                 }
    //             } else {
    //                 new_node.val = v1.unwrap();
    //                 list1 = list1.unwrap().next;
    //             }
    //         } else if v2 != None {
    //             new_node.val = v2.unwrap();
    //             list2 = list2.unwrap().next;
    //         } else {
    //             break;
    //         }

    //         match result {
    //             None => {
    //                 result = Some(new_node);
    //                 root = Some(new_node);
    //             }
    //             Some(curr_node) => {
    //                 curr_node.next = Some(*new_node.as_ref());
    //                 result = Some(new_node);
    //             }
    //         }
    //     }

    //     return Some(*root.unwrap().as_ref());
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        // let list1 = Some(Box::new(ListNode::new(
        //     4,
        //     Some(Box::new(ListNode::new(5, None))),
        // )));
        // let list2 = Some(Box::new(ListNode::new(
        //     1,
        //     Some(Box::new(ListNode::new(6, None))),
        // )));

        // println!(
        //     "{:?}",
        //     Solution::merge_two_lists(list1.clone(), list2.clone())
        // );
        // println!("List1 : {:?}", list1);
        // println!("List2 : {:?}", list2);
    }
}
