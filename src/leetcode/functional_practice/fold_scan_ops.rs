pub fn demonstrate_fold_scan_operations() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    // 1. fold - 基本折叠操作
    let sum = numbers.iter()
        .fold(0, |acc, x| acc + x);
    println!("Sum using fold: {}", sum);
    
    // 2. fold with string concatenation
    let string_concat = numbers.iter()
        .fold(String::new(), |acc, x| acc + &x.to_string());
    println!("String concatenation: {}", string_concat);
    
    // 3. scan - 保留中间状态
    let running_sum: Vec<i32> = numbers.iter()
        .scan(0, |acc, &x| {
            *acc += x;
            Some(*acc)
        })
        .collect();
    println!("Running sum using scan: {:?}", running_sum);
    
    // 4. reduce - 使用第一个元素作为初始值
    let product = numbers.iter()
        .copied()
        .reduce(|acc, x| acc * x);
    println!("Product using reduce: {:?}", product);
    
    // 5. 复杂的fold示例 - 计算统计信息
    let stats = numbers.iter()
        .fold((0, 0, 1), |(count, sum, product), &x| {
            (count + 1, sum + x, product * x)
        });
    println!("Statistics (count, sum, product): {:?}", stats);
    
    // 6. scan with filtering
    let fibonacci: Vec<i32> = vec![1; 10].iter()
        .scan((0, 1), |state, _| {
            let next = state.0 + state.1;
            *state = (state.1, next);
            Some(next)
        })
        .collect();
    println!("Fibonacci sequence using scan: {:?}", fibonacci);
    
    // 7. fold用于构建复杂数据结构
    use std::collections::HashMap;
    let char_count: HashMap<char, i32> = "hello world"
        .chars()
        .fold(HashMap::new(), |mut map, c| {
            *map.entry(c).or_insert(0) += 1;
            map
        });
    println!("Character count using fold: {:?}", char_count);
    
    // 8. 组合scan和其他操作
    let running_average: Vec<f64> = numbers.iter()
        .scan((0, 0), |state, &x| {
            state.0 += x;    // sum
            state.1 += 1;    // count
            Some(state.0 as f64 / state.1 as f64)
        })
        .collect();
    println!("Running average: {:?}", running_average);
} 