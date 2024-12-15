pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    let mut nums = nums;
    nums.sort();
    let mut closest_sum = nums[0] + nums[1] + nums[2];
    
    for i in 0..nums.len() - 2 {
        let mut left = i + 1;
        let mut right = nums.len() - 1;
        
        while left < right {
            let current_sum = nums[i] + nums[left] + nums[right];
            
            if current_sum == target {
                return current_sum;
            }
            
            if (current_sum - target).abs() < (closest_sum - target).abs() {
                closest_sum = current_sum;
            }
            
            if current_sum < target {
                left += 1;
            } else {
                right -= 1;
            }
        }
    }
    
    closest_sum
} 