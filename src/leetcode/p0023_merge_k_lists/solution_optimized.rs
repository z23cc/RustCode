use crate::leetcode::p0002_add_two_numbers::list_node::ListNode;
use std::collections::BinaryHeap;
use std::cmp::{Ordering, Reverse};

// 包装类型，同时存储值和下一个节点
#[derive(Eq, PartialEq)]
struct HeapNode {
    val: i32,
    index: usize,
    node: Option<Box<ListNode>>,
}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.cmp(&other.val)
            .then(self.index.cmp(&other.index))
    }
}

pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    // 创建最小堆
    let mut heap = BinaryHeap::with_capacity(lists.len());
    
    // 将所有链表的第一个节点加入堆
    for (i, node) in lists.into_iter().enumerate() {
        if let Some(node) = node {
            heap.push(Reverse(HeapNode {
                val: node.val,
                index: i,
                node: Some(node),
            }));
        }
    }
    
    let mut dummy = Box::new(ListNode::new(0));
    let mut curr = &mut dummy;
    
    // 不断从堆中取出最小值
    while let Some(Reverse(mut heap_node)) = heap.pop() {
        if let Some(mut node) = heap_node.node.take() {
            // 如果当前节点有下一个节点，将其加入堆
            if let Some(next) = node.next.take() {
                heap.push(Reverse(HeapNode {
                    val: next.val,
                    index: heap_node.index,
                    node: Some(next),
                }));
            }
            
            // 将当前节点添加到结果链表
            curr.next = Some(node);
            curr = curr.next.as_mut().unwrap();
        }
    }
    
    dummy.next
} 