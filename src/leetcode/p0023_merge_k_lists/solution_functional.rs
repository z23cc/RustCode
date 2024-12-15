use crate::leetcode::p0002_add_two_numbers::list_node::ListNode;

pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    // 将所有节点收集到一个Vec中
    let mut all_values: Vec<i32> = lists.iter()
        .filter_map(|list| list.as_ref())
        .flat_map(|head| {
            std::iter::successors(
                Some(head),
                |node| node.next.as_ref()
            )
            .map(|node| node.val)
        })
        .collect();
    
    // 排序
    all_values.sort_unstable();
    
    // 构建新链表
    all_values.into_iter()
        .rev()
        .fold(None, |acc, val| {
            Some(Box::new(ListNode {
                val,
                next: acc
            }))
        })
} 