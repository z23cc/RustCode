use std::collections::{HashMap, HashSet};

pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    if nums.len() < 3 {
        return vec![];
    }
    
    let mut num_count = HashMap::new();
    for &num in nums.iter() {
        *num_count.entry(num).or_insert(0) += 1;
    }
    
    let mut unique_nums: Vec<i32> = num_count.keys().cloned().collect();
    unique_nums.sort();
    let mut result = HashSet::new();
    
    // 处理特殊情况：三个0
    if num_count.get(&0).unwrap_or(&0) >= &3 {
        result.insert(vec![0, 0, 0]);
    }
    
    // 处理两个相同的数
    for &num in unique_nums.iter() {
        if num_count.get(&num).unwrap_or(&0) >= &2 {
            let target = -2 * num;
            if target != num && num_count.contains_key(&target) {
                if target < num {
                    result.insert(vec![target, num, num]);
                } else {
                    result.insert(vec![num, num, target]);
                }
            }
        }
    }
    
    // 处理三个不同的数
    for i in 0..unique_nums.len() {
        for j in i + 1..unique_nums.len() {
            let a = unique_nums[i];
            let b = unique_nums[j];
            let c = -(a + b);
            
            if c > b && num_count.contains_key(&c) {
                result.insert(vec![a, b, c]);
            }
        }
    }
    
    // 将结果转换为Vec并排序
    let mut result: Vec<Vec<i32>> = result.into_iter().collect();
    result.sort();
    result
} 