pub fn reverse(x: i32) -> i32 {
    // 特殊处理 i32::MIN
    if x == i32::MIN {
        return 0;  // 反转后必然溢出
    }
    
    x.abs()
        .to_string()
        .chars()
        .rev()
        .collect::<String>()
        .parse::<i32>()
        .map(|n| n * x.signum())
        .unwrap_or(0)
} 