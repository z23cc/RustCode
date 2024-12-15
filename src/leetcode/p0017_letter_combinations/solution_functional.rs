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
    
    digits.bytes()
        .map(|b| digit_map[(b - b'0') as usize].chars().collect::<Vec<_>>())
        .fold(vec![String::new()], |acc, chars| {
            acc.into_iter()
                .flat_map(|s| {
                    chars.iter().map(move |&c| {
                        let mut new_s = s.clone();
                        new_s.push(c);
                        new_s
                    })
                })
                .collect()
        })
} 