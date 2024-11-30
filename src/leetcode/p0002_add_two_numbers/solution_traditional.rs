use super::list_node::ListNode;

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>
) -> Option<Box<ListNode>> {
    let mut dummy = ListNode::new(0);
    let mut current = &mut dummy;
    let mut carry = 0;
    let mut p1 = l1;
    let mut p2 = l2;

    while p1.is_some() || p2.is_some() || carry != 0 {
        let sum = carry
            + p1.as_ref().map_or(0, |n| n.val)
            + p2.as_ref().map_or(0, |n| n.val);
        
        carry = sum / 10;
        current.next = Some(Box::new(ListNode::new(sum % 10)));
        current = current.next.as_mut().unwrap();
        
        p1 = p1.and_then(|n| n.next);
        p2 = p2.and_then(|n| n.next);
    }

    dummy.next
} 