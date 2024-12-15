pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let n = n as usize;
    let mut dp = vec![Vec::new(); n + 1];
    dp[0].push(String::new());
    
    for i in 1..=n {
        let mut current_set = Vec::new();
        for j in 0..i {
            let k = i - 1 - j;
            
            // 克隆需要的字符串
            let left_strings = dp[j].clone();
            let right_strings = dp[k].clone();
            
            // 生成所有可能的组合
            for left_str in left_strings {
                for right_str in right_strings.iter() {
                    current_set.push(format!("({}){}", left_str, right_str));
                }
            }
        }
        dp[i] = current_set;
    }
    
    dp[n].clone()
} 