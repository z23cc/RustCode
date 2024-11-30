pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut merged = Vec::with_capacity(nums1.len() + nums2.len());
    let mut i = 0;
    let mut j = 0;
    
    // 合并两个有序数组
    while i < nums1.len() && j < nums2.len() {
        if nums1[i] <= nums2[j] {
            merged.push(nums1[i]);
            i += 1;
        } else {
            merged.push(nums2[j]);
            j += 1;
        }
    }
    
    // 处理剩余元素
    merged.extend_from_slice(&nums1[i..]);
    merged.extend_from_slice(&nums2[j..]);
    
    // 计算中位数
    let len = merged.len();
    if len % 2 == 0 {
        (merged[len/2 - 1] + merged[len/2]) as f64 / 2.0
    } else {
        merged[len/2] as f64
    }
} 