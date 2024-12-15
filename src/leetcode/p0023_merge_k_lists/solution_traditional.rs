use crate::leetcode::p0002_add_two_numbers::list_node::ListNode;

pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    if lists.is_empty() {
        return None;
    }
    
    // 合并两个有序链表
    fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (l1, l2) {
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
    
    // 分治合并
    fn merge_lists(lists: &[Option<Box<ListNode>>], start: usize, end: usize) -> Option<Box<ListNode>> {
        match end - start {
            0 => None,
            1 => lists[start].clone(),
            2 => merge_two_lists(lists[start].clone(), lists[start + 1].clone()),
            n => {
                let mid = start + n / 2;
                let left = merge_lists(lists, start, mid);
                let right = merge_lists(lists, mid, end);
                merge_two_lists(left, right)
            }
        }
    }
    
    merge_lists(&lists, 0, lists.len())
} 