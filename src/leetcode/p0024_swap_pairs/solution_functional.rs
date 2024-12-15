use crate::leetcode::p0002_add_two_numbers::list_node::ListNode;

pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match head {
        None => None,
        Some(mut node) => match node.next.take() {
            None => Some(node),
            Some(mut next) => {
                // 递归处理剩余部分
                node.next = swap_pairs(next.next.take());
                next.next = Some(node);
                Some(next)
            }
        }
    }
} 