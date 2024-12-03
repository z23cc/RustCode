use std::collections::HashSet;

pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    if nums.len() < 3 {
        return vec![];
    }
    
    let mut nums = nums;
    nums.sort();
    
    let result: HashSet<Vec<i32>> = (0..nums.len().saturating_sub(2))
        .filter(|&i| i == 0 || nums[i] != nums[i - 1])
        .flat_map(|i| {
            let first = nums[i];
            let mut j = i + 1;
            let mut k = nums.len() - 1;
            let mut triplets = Vec::new();
            
            while j < k {
                let sum = first + nums[j] + nums[k];
                match sum.cmp(&0) {
                    std::cmp::Ordering::Equal => {
                        triplets.push(vec![first, nums[j], nums[k]]);
                        while j < k && nums[j] == nums[j + 1] { j += 1; }
                        while j < k && nums[k] == nums[k - 1] { k -= 1; }
                        j += 1;
                        k -= 1;
                    }
                    std::cmp::Ordering::Less => j += 1,
                    std::cmp::Ordering::Greater => k -= 1,
                }
            }
            triplets
        })
        .collect();
    
    result.into_iter().collect()
} 