pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    let mut nums = nums;
    nums.sort();
    let mut closest_sum = nums[0] + nums[1] + nums[2];
    
    for i in 0..nums.len() - 2 {
        // 如果当前数字和前一个相同，跳过以避免重复计算
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }
        
        let mut left = i + 1;
        let mut right = nums.len() - 1;
        
        // 计算当前位置的最小和最大可能和
        let min_sum = nums[i] + nums[i + 1] + nums[i + 2];
        if min_sum > target {
            // 如果最小和已经大于目标，更新closest_sum并退出
            return if (min_sum - target).abs() < (closest_sum - target).abs() {
                min_sum
            } else {
                closest_sum
            };
        }
        
        let max_sum = nums[i] + nums[nums.len() - 2] + nums[nums.len() - 1];
        if max_sum < target {
            // 如果最大和小于目标，更新closest_sum并继续下一轮
            if (max_sum - target).abs() < (closest_sum - target).abs() {
                closest_sum = max_sum;
            }
            continue;
        }
        
        while left < right {
            let current_sum = nums[i] + nums[left] + nums[right];
            
            if current_sum == target {
                return target;
            }
            
            if (current_sum - target).abs() < (closest_sum - target).abs() {
                closest_sum = current_sum;
            }
            
            if current_sum < target {
                left += 1;
                // 跳过重复元素
                while left < right && nums[left] == nums[left - 1] {
                    left += 1;
                }
            } else {
                right -= 1;
                // 跳过重复元素
                while left < right && nums[right] == nums[right + 1] {
                    right -= 1;
                }
            }
        }
    }
    
    closest_sum
} 