use std::collections::HashSet;

pub fn length_of_longest_substring(s: String) -> i32 {
    let chars: Vec<char> = s.chars().collect();
    let mut set = HashSet::new();
    let mut max_len = 0;
    let mut left = 0;
    let mut right = 0;

    while right < chars.len() {
        while set.contains(&chars[right]) {
            set.remove(&chars[left]);
            left += 1;
        }
        set.insert(chars[right]);
        max_len = max_len.max(right - left + 1);
        right += 1;
    }

    max_len as i32
} 