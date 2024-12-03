pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::new();
    }
    
    let first = &strs[0];
    let mut prefix_len = first.len();
    
    for str in strs.iter().skip(1) {
        let mut i = 0;
        while i < prefix_len && i < str.len() {
            if first.as_bytes()[i] != str.as_bytes()[i] {
                break;
            }
            i += 1;
        }
        prefix_len = prefix_len.min(i);
    }
    
    first[..prefix_len].to_string()
} 