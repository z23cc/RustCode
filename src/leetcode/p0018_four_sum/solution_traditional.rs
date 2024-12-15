pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    if nums.len() < 4 {
        return result;
    }
    
    let mut nums = nums;
    nums.sort();
    let target = target as i64; // 防止溢出
    
    for i in 0..nums.len() - 3 {
        // 跳过重复的第一个数
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }
        
        for j in i + 1..nums.len() - 2 {
            // 跳过重复的第二个数
            if j > i + 1 && nums[j] == nums[j - 1] {
                continue;
            }
            
            let mut left = j + 1;
            let mut right = nums.len() - 1;
            
            while left < right {
                let sum = nums[i] as i64 + nums[j] as i64 + 
                         nums[left] as i64 + nums[right] as i64;
                
                match sum.cmp(&target) {
                    std::cmp::Ordering::Equal => {
                        result.push(vec![nums[i], nums[j], nums[left], nums[right]]);
                        
                        // 跳过重复的第三个数和第四个数
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
    }
    
    result
} 