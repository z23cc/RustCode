pub fn int_to_roman(num: i32) -> String {
    let mut result = String::with_capacity(20);
    let mut n = num;
    
    // 处理千位
    let m = n / 1000;
    result.push_str(&"M".repeat(m as usize));
    n %= 1000;
    
    // 处理百位
    match n / 100 {
        9 => result.push_str("CM"),
        4 => result.push_str("CD"),
        x if x >= 5 => {
            result.push('D');
            result.push_str(&"C".repeat(x as usize - 5));
        },
        x => result.push_str(&"C".repeat(x as usize)),
    }
    n %= 100;
    
    // 处理十位
    match n / 10 {
        9 => result.push_str("XC"),
        4 => result.push_str("XL"),
        x if x >= 5 => {
            result.push('L');
            result.push_str(&"X".repeat(x as usize - 5));
        },
        x => result.push_str(&"X".repeat(x as usize)),
    }
    n %= 10;
    
    // 处理个位
    match n {
        9 => result.push_str("IX"),
        4 => result.push_str("IV"),
        x if x >= 5 => {
            result.push('V');
            result.push_str(&"I".repeat(x as usize - 5));
        },
        x => result.push_str(&"I".repeat(x as usize)),
    }
    
    result
} 