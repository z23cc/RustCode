use std::collections::HashMap;

pub fn length_of_longest_substring(s: String) -> i32 {
    s.chars()
        .enumerate()
        .fold(
            (HashMap::new(), 0, 0), // (last_pos, current_start, max_len)
            |(mut pos_map, start, max_len), (i, c)| {
                let new_start = if let Some(&last_pos) = pos_map.get(&c) {
                    start.max(last_pos + 1)
                } else {
                    start
                };
                
                pos_map.insert(c, i);
                (pos_map, new_start, max_len.max(i - new_start + 1))
            }
        )
        .2 as i32
} 