pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let (nums1, nums2) = if nums1.len() > nums2.len() {
        (nums2, nums1)
    } else {
        (nums1, nums2)
    };
    
    let m = nums1.len();
    let n = nums2.len();
    let total = m + n;
    let half = (total + 1) / 2;
    
    let mut left = 0;
    let mut right = m;
    
    while left <= right {
        let i = (left + right) / 2;
        let j = half - i;
        
        let nums1_left = if i == 0 { i32::MIN } else { nums1[i-1] };
        let nums1_right = if i == m { i32::MAX } else { nums1[i] };
        let nums2_left = if j == 0 { i32::MIN } else { nums2[j-1] };
        let nums2_right = if j == n { i32::MAX } else { nums2[j] };
        
        if nums1_left <= nums2_right && nums2_left <= nums1_right {
            // 找到正确的分割点
            if total % 2 == 0 {
                let left_max = nums1_left.max(nums2_left);
                let right_min = nums1_right.min(nums2_right);
                return (left_max + right_min) as f64 / 2.0;
            } else {
                return nums1_left.max(nums2_left) as f64;
            }
        } else if nums1_left > nums2_right {
            right = i - 1;
        } else {
            left = i + 1;
        }
    }
    
    unreachable!()
} 