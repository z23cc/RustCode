pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let merged: Vec<i32> = nums1.iter()
        .cloned()
        .chain(nums2.iter().cloned())
        .collect::<Vec<_>>()
        .into_iter()
        .fold(Vec::new(), |mut acc, x| {
            let pos = acc.binary_search(&x).unwrap_or_else(|e| e);
            acc.insert(pos, x);
            acc
        });
    
    let len = merged.len();
    if len % 2 == 0 {
        merged.iter()
            .skip(len/2 - 1)
            .take(2)
            .map(|&x| x as f64)
            .sum::<f64>() / 2.0
    } else {
        merged[len/2] as f64
    }
} 