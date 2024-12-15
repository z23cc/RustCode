use crate::leetcode::p0002_add_two_numbers::list_node::ListNode;

pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match head {
        None => None,
        Some(mut first) => {
            if first.next.is_none() {
                Some(first)
            } else if let Some(mut second) = first.next.take() {
                // 保存并递归处理后续节点
                let rest = second.next.take();
                let new_rest = swap_pairs(rest);
                
                // 交换当前两个节点
                first.next = new_rest;
                second.next = Some(first);
                Some(second)
            } else {
                Some(first)
            }
        }
    }
} 