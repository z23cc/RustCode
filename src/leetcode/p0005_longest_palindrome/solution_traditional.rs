pub fn longest_palindrome(s: String) -> String {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() < 2 {
        return s;
    }
    
    let mut start = 0;
    let mut max_len = 1;
    
    // 辅助函数：从中心向两边扩展
    let expand = |left: i32, right: i32| -> (usize, usize) {
        let mut l = left;
        let mut r = right;
        while l >= 0 && r < chars.len() as i32 
            && chars[l as usize] == chars[r as usize] {
            l -= 1;
            r += 1;
        }
        ((l + 1) as usize, (r - l - 1) as usize)
    };
    
    // 遍历每个可能的中心点
    for i in 0..chars.len() {
        // 奇数长度的回文串
        let (start1, len1) = expand(i as i32, i as i32);
        if len1 > max_len {
            start = start1;
            max_len = len1;
        }
        
        // 偶数长度的回文串
        if i + 1 < chars.len() {
            let (start2, len2) = expand(i as i32, (i + 1) as i32);
            if len2 > max_len {
                start = start2;
                max_len = len2;
            }
        }
    }
    
    chars[start..start + max_len].iter().collect()
} 