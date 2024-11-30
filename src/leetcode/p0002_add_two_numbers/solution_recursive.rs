use super::list_node::ListNode;

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>
) -> Option<Box<ListNode>> {
    fn add_with_carry(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        carry: i32
    ) -> Option<Box<ListNode>> {
        match (l1, l2, carry) {
            (None, None, 0) => None,
            (l1, l2, carry) => {
                let sum = carry
                    + l1.as_ref().map_or(0, |n| n.val)
                    + l2.as_ref().map_or(0, |n| n.val);
                
                Some(Box::new(ListNode {
                    val: sum % 10,
                    next: add_with_carry(
                        l1.and_then(|n| n.next),
                        l2.and_then(|n| n.next),
                        sum / 10
                    )
                }))
            }
        }
    }

    add_with_carry(l1, l2, 0)
} 