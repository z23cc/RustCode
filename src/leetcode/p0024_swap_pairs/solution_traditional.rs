use crate::leetcode::p0002_add_two_numbers::list_node::ListNode;

pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(0));
    dummy.next = head;
    let mut curr = &mut dummy;
    
    while curr.next.is_some() && curr.next.as_ref().unwrap().next.is_some() {
        let mut first = curr.next.take().unwrap();
        let mut second = first.next.take().unwrap();
        
        // 交换节点
        first.next = second.next.take();
        second.next = Some(first);
        curr.next = Some(second);
        
        // 移动到下一对节点
        curr = curr.next.as_mut().unwrap().next.as_mut().unwrap();
    }
    
    dummy.next
} 