pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    
    x.to_string()
        .chars()
        .zip(x.to_string().chars().rev())
        .all(|(a, b)| a == b)
} 