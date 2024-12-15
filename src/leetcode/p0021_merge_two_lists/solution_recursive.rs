use crate::leetcode::p0002_add_two_numbers::list_node::ListNode;

pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    fn merge_recursive(
        l1: Option<Box<ListNode>>, 
        l2: Option<Box<ListNode>>, 
        tail: &mut Option<Box<ListNode>>
    ) {
        match (l1, l2) {
            (None, None) => return,
            (Some(node), None) | (None, Some(node)) => *tail = Some(node),
            (Some(mut node1), Some(mut node2)) => {
                if node1.val <= node2.val {
                    let next = node1.next.take();
                    *tail = Some(node1);
                    merge_recursive(next, Some(node2), &mut tail.as_mut().unwrap().next);
                } else {
                    let next = node2.next.take();
                    *tail = Some(node2);
                    merge_recursive(Some(node1), next, &mut tail.as_mut().unwrap().next);
                }
            }
        }
    }
    
    let mut result = None;
    merge_recursive(list1, list2, &mut result);
    result
} 