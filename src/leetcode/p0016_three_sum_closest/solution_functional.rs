pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    let mut nums = nums;
    nums.sort();
    
    let first_sum = nums[0] + nums[1] + nums[2];
    let nums_ref = &nums;
    
    (0..nums_ref.len().saturating_sub(2))
        .flat_map(|i| {
            (i + 1..nums_ref.len() - 1).flat_map(move |j| {
                (j + 1..nums_ref.len()).map(move |k| {
                    nums_ref[i] + nums_ref[j] + nums_ref[k]
                })
            })
        })
        .fold(first_sum, |closest, sum| {
            if (sum - target).abs() < (closest - target).abs() {
                sum
            } else {
                closest
            }
        })
} 