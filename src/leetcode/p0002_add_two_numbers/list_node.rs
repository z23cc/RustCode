#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }

    // 辅助函数：从数组创建链表
    pub fn from_array(arr: &[i32]) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut current = &mut head;
        
        for &num in arr {
            *current = Some(Box::new(ListNode::new(num)));
            current = &mut current.as_mut().unwrap().next;
        }
        
        head
    }

    // 辅助函数：将链表转换为数组
    pub fn to_array(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut current = head;
        
        while let Some(node) = current {
            result.push(node.val);
            current = node.next;
        }
        
        result
    }
} 