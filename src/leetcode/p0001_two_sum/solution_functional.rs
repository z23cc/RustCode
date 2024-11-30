use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    nums.iter()
        .enumerate()
        .scan(HashMap::new(), |map, (i, &num)| {
            if let Some(&j) = map.get(&(target - num)) {
                Some(Some(vec![j as i32, i as i32]))
            } else {
                map.insert(num, i);
                Some(None)
            }
        })
        .filter_map(|x| x)
        .next()
        .unwrap_or_default()
} 