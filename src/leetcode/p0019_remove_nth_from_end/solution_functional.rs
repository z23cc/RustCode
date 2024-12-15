use crate::leetcode::p0002_add_two_numbers::list_node::ListNode;

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    // 将链表转换为Vec
    let mut nodes: Vec<_> = std::iter::successors(head.as_ref(), |node| node.next.as_ref())
        .collect();
    
    // 如果链表为空或n超出范围，直接返回
    if nodes.is_empty() || n as usize > nodes.len() {
        return head;
    }
    
    // 重建链表，跳过倒数第n个节点
    let len = nodes.len();
    (0..len)
        .filter(|&i| i != len - n as usize)
        .rev()
        .fold(None, |acc, i| {
            let mut new_node = Box::new(ListNode::new(nodes[i].val));
            new_node.next = acc;
            Some(new_node)
        })
} 