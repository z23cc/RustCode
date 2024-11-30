pub fn reverse(x: i32) -> i32 {
    // 特殊处理 i32::MIN
    if x == i32::MIN {
        return 0;  // 反转后必然溢出
    }
    
    let is_negative = x < 0;
    let mut s = x.abs().to_string();
    
    // 原地反转字符串
    unsafe {
        let bytes = s.as_bytes_mut();
        let len = bytes.len();
        for i in 0..len/2 {
            bytes.swap(i, len-1-i);
        }
    }
    
    // 转换回数字，处理溢出
    match s.parse::<i32>() {
        Ok(n) if is_negative => Some(-n),
        Ok(n) => Some(n),
        Err(_) => None,
    }.unwrap_or(0)
} 