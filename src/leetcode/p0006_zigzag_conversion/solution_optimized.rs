pub fn convert(s: String, num_rows: i32) -> String {
    if num_rows == 1 {
        return s;
    }
    
    let n = s.len();
    let num_rows = num_rows as usize;
    let cycle = 2 * num_rows - 2;
    let mut result = String::with_capacity(n);
    let chars: Vec<char> = s.chars().collect();
    
    // 第一行
    for i in (0..n).step_by(cycle) {
        result.push(chars[i]);
    }
    
    // 中间行
    for row in 1..num_rows-1 {
        let mut i = row;
        while i < n {
            result.push(chars[i]);
            let next = i + cycle - 2 * row;
            if next < n {
                result.push(chars[next]);
            }
            i += cycle;
        }
    }
    
    // 最后一行
    for i in (num_rows-1..n).step_by(cycle) {
        result.push(chars[i]);
    }
    
    result
} 