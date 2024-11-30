pub fn my_atoi(s: String) -> i32 {
    let mut chars = s.trim_start().chars();
    let mut result: i64 = 0;
    let mut is_negative = false;
    
    // 处理符号
    if let Some(first) = chars.next() {
        match first {
            '-' => is_negative = true,
            '+' => {},
            '0'..='9' => result = (first as u8 - b'0') as i64,
            _ => return 0,
        }
    } else {
        return 0;
    }
    
    // 处理数字
    for c in chars {
        if !c.is_ascii_digit() {
            break;
        }
        
        result = result * 10 + (c as u8 - b'0') as i64;
        if result > i32::MAX as i64 {
            return if is_negative { i32::MIN } else { i32::MAX };
        }
    }
    
    // 应用符号并检查范围
    result = if is_negative { -result } else { result };
    if result < i32::MIN as i64 {
        i32::MIN
    } else if result > i32::MAX as i64 {
        i32::MAX
    } else {
        result as i32
    }
} 