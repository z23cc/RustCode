pub fn longest_palindrome(s: String) -> String {
    if s.is_empty() {
        return String::new();
    }
    
    // 预处理字符串，插入特殊字符
    let chars: Vec<char> = s.chars().collect();
    let processed: Vec<char> = chars.iter()
        .flat_map(|&c| vec!['#', c])
        .chain(std::iter::once('#'))
        .collect();
    
    let n = processed.len();
    let mut p = vec![0; n];  // 回文半径数组
    let mut center = 0;      // 当前中心
    let mut max_right = 0;   // 当前最右边界
    let mut max_len = 0;     // 最长回文串长度
    let mut max_center = 0;  // 最长回文串中心
    
    for i in 0..n {
        if i < max_right {
            let mirror = 2 * center - i;
            p[i] = (max_right - i).min(p[mirror]);
        }
        
        // 中心扩展
        let (mut left, mut right) = (i as i32 - p[i] as i32 - 1, i + p[i] + 1);
        while left >= 0 && right < n 
            && processed[left as usize] == processed[right] {
            p[i] += 1;
            left -= 1;
            right += 1;
        }
        
        // 更新中心和右边界
        if i + p[i] > max_right {
            center = i;
            max_right = i + p[i];
        }
        
        // 更新最长回文串信息
        if p[i] > max_len {
            max_len = p[i];
            max_center = i;
        }
    }
    
    // 还原原始字符串的回文子串
    let start = (max_center - max_len) / 2;
    chars[start..start + max_len].iter().collect()
} 