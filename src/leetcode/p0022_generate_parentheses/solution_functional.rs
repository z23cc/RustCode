pub fn generate_parenthesis(n: i32) -> Vec<String> {
    fn generate(open: i32, close: i32, max: i32, current: String) -> Vec<String> {
        if current.len() == (max * 2) as usize {
            return vec![current];
        }
        
        let mut result = Vec::new();
        
        if open < max {
            result.extend(generate(
                open + 1, 
                close, 
                max, 
                format!("{}(", current)
            ));
        }
        
        if close < open {
            result.extend(generate(
                open, 
                close + 1, 
                max, 
                format!("{})", current)
            ));
        }
        
        result
    }
    
    generate(0, 0, n, String::new())
} 