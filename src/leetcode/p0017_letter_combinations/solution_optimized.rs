pub fn letter_combinations(digits: String) -> Vec<String> {
    if digits.is_empty() {
        return vec![];
    }
    
    let digit_map = [
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
    
    let mut result = vec![String::new()];
    
    for digit in digits.bytes() {
        let digit = (digit - b'0') as usize;
        let mut new_result = Vec::with_capacity(result.len() * digit_map[digit].len());
        
        for combination in result {
            for c in digit_map[digit].chars() {
                let mut new_combination = combination.clone();
                new_combination.push(c);
                new_result.push(new_combination);
            }
        }
        
        result = new_result;
    }
    
    result
} 