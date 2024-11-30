pub fn demonstrate_transform_operations() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    // 1. map - 转换每个元素
    let doubled: Vec<i32> = numbers.iter()
        .map(|x| x * 2)
        .collect();
    println!("Doubled: {:?}", doubled);
    
    // 2. filter_map - 转换并可能过滤掉元素
    let even_doubled: Vec<i32> = numbers.iter()
        .filter_map(|x| {
            if x % 2 == 0 {
                Some(x * 2)
            } else {
                None
            }
        })
        .collect();
    println!("Even doubled: {:?}", even_doubled);
    
    // 3. flat_map - 展平嵌套的迭代器
    let nested = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    let flattened: Vec<i32> = nested.iter()
        .flat_map(|x| x.iter().cloned())
        .collect();
    println!("Flattened: {:?}", flattened);
    
    // 4. map与其他组合使用
    let complex: Vec<String> = numbers.iter()
        .map(|x| x * 3)
        .filter(|x| x > &5)
        .map(|x| format!("num_{}", x))
        .collect();
    println!("Complex transform: {:?}", complex);
    
    // 5. 字符串转换示例
    let words = vec!["hello", "world"];
    let chars: Vec<char> = words.iter()
        .flat_map(|s| s.chars())
        .collect();
    println!("Characters: {:?}", chars);
    
    // 6. 链式转换
    let result: Vec<i32> = numbers.iter()
        .map(|x| x * 2)
        .filter(|x| x > &5)
        .map(|x| x + 1)
        .collect();
    println!("Chained transforms: {:?}", result);
} 