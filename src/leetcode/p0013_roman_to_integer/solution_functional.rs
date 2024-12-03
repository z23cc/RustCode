use std::collections::HashMap;

pub fn roman_to_int(s: String) -> i32 {
    let values: HashMap<char, i32> = [
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ].iter().cloned().collect();
    
    s.chars()
        .rev()
        .scan(0, |prev_value, c| {
            let current_value = values.get(&c).unwrap_or(&0);
            let result = if current_value >= prev_value {
                *current_value
            } else {
                -current_value
            };
            *prev_value = *current_value;
            Some(result)
        })
        .sum()
} 