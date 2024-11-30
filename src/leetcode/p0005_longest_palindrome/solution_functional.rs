pub fn longest_palindrome(s: String) -> String {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() < 2 {
        return s;
    }
    
    // 生成所有可能的中心点
    let centers = (0..chars.len())
        .flat_map(|i| vec![(i as i32, i as i32), (i as i32, (i + 1) as i32)])
        .filter(|&(_, r)| r < chars.len() as i32);
    
    // 从中心扩展并找到最长回文串
    centers
        .map(|(left, right)| {
            let mut l = left;
            let mut r = right;
            while l >= 0 && r < chars.len() as i32 
                && chars[l as usize] == chars[r as usize] {
                l -= 1;
                r += 1;
            }
            ((l + 1) as usize, (r - l - 1) as usize)
        })
        .max_by_key(|&(_, len)| len)
        .map(|(start, len)| chars[start..start + len].iter().collect())
        .unwrap_or_else(|| s.chars().next().unwrap().to_string())
} 