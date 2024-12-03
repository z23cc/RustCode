pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::new();
    }
    
    let min_len = strs.iter()
        .map(|s| s.len())
        .min()
        .unwrap_or(0);
    
    (0..min_len)
        .take_while(|&i| {
            let ch = strs[0].as_bytes()[i];
            strs.iter()
                .all(|s| s.as_bytes()[i] == ch)
        })
        .map(|i| strs[0].as_bytes()[i] as char)
        .collect()
} 