use std::collections::HashMap;

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut char_index = HashMap::with_capacity(128);
    let mut max_len = 0;
    let mut start = 0;

    s.chars()
        .enumerate()
        .for_each(|(i, c)| {
            if let Some(&prev_i) = char_index.get(&c) {
                if prev_i >= start {
                    start = prev_i + 1;
                }
            }
            max_len = max_len.max(i - start + 1);
            char_index.insert(c, i);
        });

    max_len as i32
} 