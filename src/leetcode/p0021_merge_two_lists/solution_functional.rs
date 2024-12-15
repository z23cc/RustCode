use crate::leetcode::p0002_add_two_numbers::list_node::ListNode;

pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match (list1, list2) {
        (None, None) => None,
        (Some(l), None) | (None, Some(l)) => Some(l),
        (Some(mut l1), Some(mut l2)) => {
            if l1.val <= l2.val {
                l1.next = merge_two_lists(l1.next.take(), Some(l2));
                Some(l1)
            } else {
                l2.next = merge_two_lists(Some(l1), l2.next.take());
                Some(l2)
            }
        }
    }
} 