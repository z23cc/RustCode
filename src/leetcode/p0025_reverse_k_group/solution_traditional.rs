use crate::leetcode::p0002_add_two_numbers::list_node::ListNode;

pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    if k <= 1 {
        return head;
    }
    
    let mut dummy = Box::new(ListNode::new(0));
    dummy.next = head;
    let mut dummy = Some(dummy);
    let mut prev = &mut dummy;
    
    while prev.is_some() {
        // 检查剩余节点是否足够k个
        let mut count = 0;
        let mut check = prev.as_ref().unwrap().next.as_ref();
        while count < k && check.is_some() {
            check = check.unwrap().next.as_ref();
            count += 1;
        }
        if count < k {
            break;
        }
        
        // 获取当前组的第一个节点
        let mut current: Option<Box<ListNode>> = prev.as_mut().unwrap().next.take();
        if current.is_none() {
            break;
        }
        
        // 反转k个节点
        let mut reversed: Option<Box<ListNode>> = None;
        for _ in 0..k {
            if let Some(mut node) = current {
                current = node.next.take();
                node.next = reversed;
                reversed = Some(node);
            }
        }
        
        // 找到反转后的尾节点
        if let Some(mut head) = reversed {
            let mut tail = &mut head;
            while tail.next.is_some() {
                tail = tail.next.as_mut().unwrap();
            }
            tail.next = current;
            prev.as_mut().unwrap().next = Some(head);
            
            // 移动到下一组的前一个节点
            for _ in 0..k {
                if let Some(node) = prev {
                    prev = &mut node.next;
                } else {
                    break;
                }
            }
        }
    }
    
    dummy.unwrap().next
} 