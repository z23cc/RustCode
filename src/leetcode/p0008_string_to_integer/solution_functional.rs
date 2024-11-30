pub fn my_atoi(s: String) -> i32 {
    let s = s.trim_start();
    
    // 提取符号和数字部分
    let (sign, digits) = s.chars()
        .take(1)
        .chain(s.chars().skip(1).take_while(|c| c.is_ascii_digit()))
        .fold((1, 0i64), |(sign, num), c| {
            match c {
                '-' if num == 0 => (-1, 0),
                '+' if num == 0 => (1, 0),
                '0'..='9' => (sign, num * 10 + (c as u8 - b'0') as i64),
                _ => (sign, num),
            }
        });
    
    // 应用符号并处理溢出
    let result = sign * digits;
    if result < i32::MIN as i64 {
        i32::MIN
    } else if result > i32::MAX as i64 {
        i32::MAX
    } else {
        result as i32
    }
} 