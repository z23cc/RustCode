pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::new();
    }
    
    fn common_prefix(s1: &str, s2: &str) -> String {
        let mut i = 0;
        let len = s1.len().min(s2.len());
        let s1_bytes = s1.as_bytes();
        let s2_bytes = s2.as_bytes();
        
        while i < len && s1_bytes[i] == s2_bytes[i] {
            i += 1;
        }
        
        s1[..i].to_string()
    }
    
    fn divide_and_conquer(strs: &[String], start: usize, end: usize) -> String {
        if start == end {
            return strs[start].clone();
        }
        
        let mid = (start + end) / 2;
        let left = divide_and_conquer(strs, start, mid);
        let right = divide_and_conquer(strs, mid + 1, end);
        
        common_prefix(&left, &right)
    }
    
    divide_and_conquer(&strs, 0, strs.len() - 1)
} 