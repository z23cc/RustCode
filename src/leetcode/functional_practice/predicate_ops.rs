pub fn demonstrate_predicate_operations() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let empty_vec: Vec<i32> = vec![];
    
    // 1. any - 检查是否存在满足条件的元素
    let has_even = numbers.iter().any(|x| x % 2 == 0);
    let has_greater_than_10 = numbers.iter().any(|x| *x > 10);
    println!("Has even numbers: {}", has_even);
    println!("Has numbers > 10: {}", has_greater_than_10);
    
    // 2. all - 检查是否所有元素都满足条件
    let all_positive = numbers.iter().all(|x| *x > 0);
    let all_less_than_5 = numbers.iter().all(|x| *x < 5);
    println!("All numbers are positive: {}", all_positive);
    println!("All numbers < 5: {}", all_less_than_5);
    
    // 3. 组合判断条件
    let complex_condition = numbers.iter()
        .any(|x| *x % 2 == 0 && *x > 5);
    println!("Has even numbers greater than 5: {}", complex_condition);
    
    // 4. is_empty 和 len 判断
    println!("Numbers is empty: {}", numbers.is_empty());
    println!("Empty vec is empty: {}", empty_vec.is_empty());
    println!("Numbers length: {}", numbers.len());
    
    // 5. 使用filter后的判断
    let filtered_any = numbers.iter()
        .filter(|x| **x > 5)
        .any(|x| *x % 2 == 0);
    println!("Has even numbers after filtering > 5: {}", filtered_any);
    
    // 6. 范围判断
    let in_range = numbers.iter()
        .all(|x| *x >= 1 && *x <= 10);
    println!("All numbers in range [1, 10]: {}", in_range);
    
    // 7. 自定义复杂判断
    let has_consecutive = numbers.windows(2)
        .any(|w| w[1] - w[0] == 1);
    println!("Has consecutive numbers: {}", has_consecutive);
    
    // 8. 多重条件组合
    let complex_check = numbers.iter()
        .filter(|x| **x > 5)
        .all(|x| *x % 2 == 0 || *x % 3 == 0);
    println!("All numbers > 5 are divisible by 2 or 3: {}", complex_check);
    
    // 9. 使用zip进行配对判断
    let pairs_ascending = numbers.windows(2)
        .all(|w| w[0] < w[1]);
    println!("Numbers are in ascending order: {}", pairs_ascending);
    
    // 在函数末尾添加测试断言
    assert!(has_even);
    assert!(!has_greater_than_10);
    assert!(all_positive);
    assert!(!all_less_than_5);
    assert!(in_range);
    assert!(has_consecutive);
    assert!(pairs_ascending);
} 