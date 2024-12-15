use std::collections::HashSet;

pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    if nums.len() < 4 {
        return vec![];
    }
    
    let mut nums = nums;
    nums.sort();
    let target = target as i64;
    
    let result: HashSet<Vec<i32>> = {
        let nums = &nums;  // 创建一个局部引用
        (0..nums.len().saturating_sub(3))
            .filter(|&i| i == 0 || nums[i] != nums[i - 1])
            .flat_map(move |i| {
                (i + 1..nums.len().saturating_sub(2))
                    .filter(move |&j| j == i + 1 || nums[j] != nums[j - 1])
                    .filter_map(move |j| {
                        let mut left = j + 1;
                        let mut right = nums.len() - 1;
                        let mut temp = Vec::new();
                        
                        while left < right {
                            let sum = nums[i] as i64 + nums[j] as i64 + 
                                    nums[left] as i64 + nums[right] as i64;
                            
                            match sum.cmp(&target) {
                                std::cmp::Ordering::Equal => {
                                    temp.push(vec![
                                        nums[i],
                                        nums[j],
                                        nums[left],
                                        nums[right]
                                    ]);
                                    left += 1;
                                    right -= 1;
                                    
                                    while left < right && nums[left] == nums[left - 1] {
                                        left += 1;
                                    }
                                    while left < right && nums[right] == nums[right + 1] {
                                        right -= 1;
                                    }
                                },
                                std::cmp::Ordering::Less => left += 1,
                                std::cmp::Ordering::Greater => right -= 1,
                            }
                        }
                        
                        if temp.is_empty() {
                            None
                        } else {
                            Some(temp)
                        }
                    })
                    .flatten()
            })
            .collect()
    };
    
    result.into_iter().collect()
} 