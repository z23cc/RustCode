use std::collections::HashMap;

pub fn is_match(s: String, p: String) -> bool {
    let s = s.as_bytes();
    let p = p.as_bytes();
    
    fn match_recursive(
        s: &[u8],
        p: &[u8],
        i: usize,
        j: usize,
        memo: &mut HashMap<(usize, usize), bool>
    ) -> bool {
        if let Some(&result) = memo.get(&(i, j)) {
            return result;
        }
        
        let result = if j == p.len() {
            i == s.len()
        } else {
            let first_match = i < s.len() && 
                (s[i] == p[j] || p[j] == b'.');
            
            if j + 1 < p.len() && p[j + 1] == b'*' {
                match_recursive(s, p, i, j + 2, memo) ||
                (first_match && match_recursive(s, p, i + 1, j, memo))
            } else {
                first_match && match_recursive(s, p, i + 1, j + 1, memo)
            }
        };
        
        memo.insert((i, j), result);
        result
    }
    
    match_recursive(s, p, 0, 0, &mut HashMap::new())
} 