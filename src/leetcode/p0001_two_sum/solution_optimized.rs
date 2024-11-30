use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::with_capacity(nums.len());
    
    nums.iter()
        .enumerate()
        .find_map(|(i, &num)| {
            map.get(&(target - num))
                .map(|&j| vec![j as i32, i as i32])
                .or_else(|| {
                    map.insert(num, i);
                    None
                })
        })
        .unwrap_or_default()
} 