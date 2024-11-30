use super::list_node::ListNode;
use std::iter;

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>
) -> Option<Box<ListNode>> {
    // 创建一个迭代器，生成链表节点的值
    let values = iter::from_fn({
        let mut l1 = l1;
        let mut l2 = l2;
        let mut carry = 0;
        move || {
            if l1.is_none() && l2.is_none() && carry == 0 {
                None
            } else {
                let sum = carry
                    + l1.as_ref().map_or(0, |n| n.val)
                    + l2.as_ref().map_or(0, |n| n.val);
                
                carry = sum / 10;
                
                if let Some(node) = l1.as_mut() {
                    l1 = node.next.take();
                }
                if let Some(node) = l2.as_mut() {
                    l2 = node.next.take();
                }
                
                Some(sum % 10)
            }
        }
    });

    // 使用 Vec 收集所有值
    let digits: Vec<_> = values.collect();
    
    // 从后向前构建链表，保持正确顺序
    digits.into_iter()
        .rev()
        .fold(None, |next, val| {
            Some(Box::new(ListNode {
                val,
                next
            }))
        })
} 