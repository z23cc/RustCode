pub fn letter_combinations(digits: String) -> Vec<String> {
    if digits.is_empty() {
        return vec![];
    }
    
    let digit_map = vec![
        "",        // 0
        "",        // 1
        "abc",     // 2
        "def",     // 3
        "ghi",     // 4
        "jkl",     // 5
        "mno",     // 6
        "pqrs",    // 7
        "tuv",     // 8
        "wxyz"     // 9
    ];
    
    let mut result = Vec::new();
    let mut current = String::new();
    
    fn backtrack(
        digits: &[u8],
        index: usize,
        current: &mut String,
        result: &mut Vec<String>,
        digit_map: &[&str]
    ) {
        if index == digits.len() {
            result.push(current.clone());
            return;
        }
        
        let digit = (digits[index] - b'0') as usize;
        for c in digit_map[digit].chars() {
            current.push(c);
            backtrack(digits, index + 1, current, result, digit_map);
            current.pop();
        }
    }
    
    backtrack(digits.as_bytes(), 0, &mut current, &mut result, &digit_map);
    result
} 