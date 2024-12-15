use std::collections::{HashMap, HashSet};

pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    if nums.len() < 4 {
        return vec![];
    }
    
    let mut nums = nums;
    nums.sort();
    let target = target as i64;
    
    // 存储所有可能的两数之和
    let mut sum_pairs = HashMap::new();
    for i in 0..nums.len() - 1 {
        for j in i + 1..nums.len() {
            let sum = nums[i] as i64 + nums[j] as i64;
            sum_pairs.entry(sum)
                .or_insert(Vec::new())
                .push((i, j));
        }
    }
    
    let mut result = HashSet::new();
    
    // 查找互补的两对数
    for (&sum1, pairs1) in sum_pairs.iter() {
        if let Some(pairs2) = sum_pairs.get(&(target - sum1)) {
            for &(i, j) in pairs1 {
                for &(k, l) in pairs2 {
                    if i < j && j < k && k < l {  // 确保不重复且有序
                        result.insert(vec![nums[i], nums[j], nums[k], nums[l]]);
                    }
                }
            }
        }
    }
    
    result.into_iter().collect()
} 