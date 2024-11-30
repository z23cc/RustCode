pub fn demonstrate_filter_operations() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // 1. filter - 基本过滤
    let evens: Vec<&i32> = numbers.iter()
        .filter(|x| *x % 2 == 0)
        .collect();
    println!("Even numbers: {:?}", evens);
    
    // 2. take - 获取前n个元素
    let first_three: Vec<&i32> = numbers.iter()
        .take(3)
        .collect();
    println!("First three: {:?}", first_three);
    
    // 3. skip - 跳过前n个元素
    let skip_first_three: Vec<&i32> = numbers.iter()
        .skip(3)
        .collect();
    println!("Skip first three: {:?}", skip_first_three);
    
    // 4. take_while - 获取元素直到条件不满足
    let less_than_five: Vec<&i32> = numbers.iter()
        .take_while(|x| **x < 5)
        .collect();
    println!("Take while less than 5: {:?}", less_than_five);
    
    // 5. skip_while - 跳过元素直到条件不满足
    let skip_until_five: Vec<&i32> = numbers.iter()
        .skip_while(|x| **x < 5)
        .collect();
    println!("Skip until 5 or greater: {:?}", skip_until_five);
    
    // 6. filter与其���组合
    let filtered_and_transformed: Vec<i32> = numbers.iter()
        .filter(|x| **x > 5)
        .map(|x| x * 2)
        .take(3)
        .collect();
    println!("Filtered and transformed: {:?}", filtered_and_transformed);
    
    // 7. 使用filter_map的高级过滤
    let strings = vec!["1", "2", "three", "4", "five", "6"];
    let parsed_numbers: Vec<i32> = strings.iter()
        .filter_map(|s| s.parse().ok())
        .collect();
    println!("Parsed numbers: {:?}", parsed_numbers);
    
    // 8. 使用步进过滤
    let step_by_two: Vec<&i32> = numbers.iter()
        .step_by(2)
        .collect();
    println!("Step by 2: {:?}", step_by_two);
    
    // 9. 组合多个过滤条件
    let complex_filter: Vec<&i32> = numbers.iter()
        .filter(|x| **x > 3)
        .filter(|x| **x < 8)
        .filter(|x| **x % 2 == 0)
        .collect();
    println!("Complex filter (3 < x < 8 and even): {:?}", complex_filter);
} 