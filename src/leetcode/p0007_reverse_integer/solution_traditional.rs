pub fn reverse(x: i32) -> i32 {
    let mut num = x;
    let mut result = 0i32;
    
    while num != 0 {
        // 检查溢出
        if result > i32::MAX / 10 || result < i32::MIN / 10 {
            return 0;
        }
        
        result = result * 10 + num % 10;
        num /= 10;
    }
    
    result
} 