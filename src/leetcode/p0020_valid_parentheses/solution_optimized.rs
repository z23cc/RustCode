pub fn is_valid(s: String) -> bool {
    if s.len() % 2 != 0 {
        return false;
    }
    
    let mut stack = Vec::with_capacity(s.len() / 2);
    
    for b in s.bytes() {
        match b {
            b'(' => stack.push(b')'),
            b'[' => stack.push(b']'),
            b'{' => stack.push(b'}'),
            b')' | b']' | b'}' => {
                if stack.pop() != Some(b) {
                    return false;
                }
            },
            _ => return false,
        }
    }
    
    stack.is_empty()
} 