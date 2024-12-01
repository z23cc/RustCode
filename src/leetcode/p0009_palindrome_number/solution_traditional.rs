pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    
    let s = x.to_string();
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    
    for i in 0..len/2 {
        if chars[i] != chars[len-1-i] {
            return false;
        }
    }
    
    true
} 