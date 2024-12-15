use crate::leetcode::p0002_add_two_numbers::list_node::ListNode;

pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    if k <= 1 {
        return head;
    }
    
    // 检查剩余节点是否足够k个
    let mut count = 0;
    let mut curr = head.as_ref();
    while count < k && curr.is_some() {
        curr = curr.unwrap().next.as_ref();
        count += 1;
    }
    
    if count < k {
        return head;
    }
    
    // 取出前k个节点并反转
    let mut curr = head;
    let mut prev = None;
    let mut next = None;
    
    // 保存第k+1个节点
    let mut count = 0;
    while let Some(mut node) = curr {
        curr = node.next.take();
        node.next = prev;
        prev = Some(node);
        count += 1;
        if count == k {
            next = curr;
            break;
        }
    }
    
    // 递归处理剩余部分
    if let Some(mut first) = prev {
        let rest = reverse_k_group(next, k);
        
        // 找到反转后的尾节点（原来的第一个节点）
        let mut tail = first.next.as_mut().unwrap();
        while tail.next.is_some() {
            tail = tail.next.as_mut().unwrap();
        }
        
        // 连接反转后的部分
        tail.next = rest;
        Some(first)
    } else {
        None
    }
} 