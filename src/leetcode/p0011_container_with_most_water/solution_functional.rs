pub fn max_area(height: Vec<i32>) -> i32 {
    let len = height.len();
    
    (0..len)
        .flat_map(|i| (i+1..len).map(move |j| (i, j)))
        .map(|(i, j)| {
            let width = (j - i) as i32;
            let height = height[i].min(height[j]);
            width * height
        })
        .max()
        .unwrap_or(0)
} 