pub fn is_match(s: String, p: String) -> bool {
    let s = s.as_bytes();
    let p = p.as_bytes();
    let m = s.len();
    let n = p.len();
    
    // dp[i][j] 表示 s[0..i] 和 p[0..j] 是否匹配
    let mut dp = vec![vec![false; n + 1]; m + 1];
    dp[0][0] = true;
    
    // 处理模式以 * 开头的情况
    for j in 1..=n {
        if p[j-1] == b'*' {
            dp[0][j] = dp[0][j-2];
        }
    }
    
    // 填充dp数组
    for i in 1..=m {
        for j in 1..=n {
            if p[j-1] == b'*' {
                dp[i][j] = dp[i][j-2] ||  // 不使用 *
                    (dp[i-1][j] && (s[i-1] == p[j-2] || p[j-2] == b'.'));  // 使用 *
            } else {
                dp[i][j] = dp[i-1][j-1] && 
                    (s[i-1] == p[j-1] || p[j-1] == b'.');
            }
        }
    }
    
    dp[m][n]
} 