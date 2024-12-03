pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    if nums.len() < 3 {
        return result;
    }
    
    let mut nums = nums;
    nums.sort();
    
    for i in 0..nums.len() - 2 {
        // 跳过重复的第一个数
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }
        
        let mut left = i + 1;
        let mut right = nums.len() - 1;
        
        while left < right {
            let sum = nums[i] + nums[left] + nums[right];
            
            match sum.cmp(&0) {
                std::cmp::Ordering::Equal => {
                    result.push(vec![nums[i], nums[left], nums[right]]);
                    
                    // 跳过重复的第二个数和第三个数
                    while left < right && nums[left] == nums[left + 1] {
                        left += 1;
                    }
                    while left < right && nums[right] == nums[right - 1] {
                        right -= 1;
                    }
                    
                    left += 1;
                    right -= 1;
                },
                std::cmp::Ordering::Less => left += 1,
                std::cmp::Ordering::Greater => right -= 1,
            }
        }
    }
    
    result
} 