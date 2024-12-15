use crate::leetcode::p0002_add_two_numbers::list_node::ListNode;

pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(0));
    let mut curr = &mut dummy;
    let mut l1 = list1;
    let mut l2 = list2;
    
    while l1.is_some() && l2.is_some() {
        let n1 = l1.as_ref().unwrap();
        let n2 = l2.as_ref().unwrap();
        
        if n1.val <= n2.val {
            curr.next = l1;
            curr = curr.next.as_mut().unwrap();
            l1 = curr.next.take();
        } else {
            curr.next = l2;
            curr = curr.next.as_mut().unwrap();
            l2 = curr.next.take();
        }
    }
    
    curr.next = if l1.is_some() { l1 } else { l2 };
    
    dummy.next
} 