pub fn convert(s: String, num_rows: i32) -> String {
    if num_rows == 1 {
        return s;
    }
    
    let num_rows = num_rows as usize;
    let cycle = 2 * num_rows - 2;
    
    (0..num_rows)
        .flat_map(|row| {
            s.chars()
                .enumerate()
                .filter_map(move |(i, c)| {
                    let pos = i % cycle;
                    if pos == row || (pos >= num_rows && pos - row == cycle - row * 2) {
                        Some(c)
                    } else {
                        None
                    }
                })
        })
        .collect()
} 