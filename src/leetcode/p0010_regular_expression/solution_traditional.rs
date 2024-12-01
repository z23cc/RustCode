pub fn is_match(s: String, p: String) -> bool {
    fn match_helper(s: &[u8], p: &[u8], i: usize, j: usize) -> bool {
        if j == p.len() {
            return i == s.len();
        }
        
        let first_match = i < s.len() && 
            (s[i] == p[j] || p[j] == b'.');
        
        if j + 1 < p.len() && p[j + 1] == b'*' {
            match_helper(s, p, i, j + 2) ||  // 不使用 *
            (first_match && match_helper(s, p, i + 1, j))  // 使用 *
        } else {
            first_match && match_helper(s, p, i + 1, j + 1)
        }
    }
    
    match_helper(s.as_bytes(), p.as_bytes(), 0, 0)
} 