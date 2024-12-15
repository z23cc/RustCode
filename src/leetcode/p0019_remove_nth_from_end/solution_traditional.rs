use crate::leetcode::p0002_add_two_numbers::list_node::ListNode;

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(0));
    dummy.next = head;
    let mut dummy = Some(dummy);
    
    // 获取链表长度
    let mut len = 0;
    let mut curr = dummy.as_ref();
    while let Some(node) = curr {
        len += 1;
        curr = node.next.as_ref();
    }
    
    // 找到要删除节点的前一个节点
    let mut curr = &mut dummy;
    for _ in 1..len-n {
        curr = &mut curr.as_mut().unwrap().next;
    }
    
    // 删除节点
    let next = curr.as_mut().unwrap().next.as_mut().unwrap().next.take();
    curr.as_mut().unwrap().next = next;
    
    dummy.unwrap().next
} 