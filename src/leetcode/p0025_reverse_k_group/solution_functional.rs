use crate::leetcode::p0002_add_two_numbers::list_node::ListNode;

pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    if k <= 1 {
        return head;
    }
    
    // 收集所有节点到向量中
    let mut nodes = Vec::new();
    while let Some(mut node) = head {
        head = node.next.take();
        nodes.push(node);
    }
    
    // 按组反转节点
    for chunk in nodes.chunks_mut(k as usize) {
        if chunk.len() == k as usize {
            chunk.reverse();
        }
    }
    
    // 重新连接节点
    let mut result = None;
    for node in nodes.into_iter().rev() {
        let mut node = node;
        node.next = result;
        result = Some(node);
    }
    
    result
} 