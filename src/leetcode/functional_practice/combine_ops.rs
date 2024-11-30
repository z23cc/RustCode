pub fn demonstrate_combine_operations() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];
    let vec3 = vec![7, 8, 9];
    
    // 1. zip - 将两个迭代器配对
    let zipped: Vec<(i32, i32)> = vec1.iter()
        .copied()
        .zip(vec2.iter().copied())
        .collect();
    println!("Zipped vectors: {:?}", zipped);
    
    // 2. zip_with - 使用自定义函数组合元素
    let sum_pairs: Vec<i32> = vec1.iter()
        .zip(vec2.iter())
        .map(|(a, b)| a + b)
        .collect();
    println!("Sum of pairs: {:?}", sum_pairs);
    
    // 3. chain - 连接多个迭代器
    let chained: Vec<i32> = vec1.iter()
        .chain(vec2.iter())
        .chain(vec3.iter())
        .copied()
        .collect();
    println!("Chained vectors: {:?}", chained);
    
    // 4. enumerate与zip组合
    let enumerated_pairs: Vec<(usize, (i32, i32))> = vec1.iter()
        .copied()
        .zip(vec2.iter().copied())
        .enumerate()
        .collect();
    println!("Enumerated pairs: {:?}", enumerated_pairs);
    
    // 5. unzip - 将配对的迭代器分开
    let pairs = vec![(1, 'a'), (2, 'b'), (3, 'c')];
    let (numbers, letters): (Vec<i32>, Vec<char>) = pairs.into_iter().unzip();
    println!("Unzipped - numbers: {:?}, letters: {:?}", numbers, letters);
    
    // 6. 交错组合
    let interleaved: Vec<i32> = vec1.iter()
        .zip(vec2.iter())
        .flat_map(|(&x, &y)| vec![x, y])
        .collect();
    println!("Interleaved vectors: {:?}", interleaved);
    
    // 7. 复杂组合示例
    let complex: Vec<(usize, (i32, i32, i32))> = vec1.iter()
        .copied()
        .zip(vec2.iter().copied())
        .zip(vec3.iter().copied())
        .map(|((a, b), c)| (a, b, c))
        .enumerate()
        .collect();
    println!("Complex combination: {:?}", complex);
    
    // 8. windows与zip组合
    let sliding_sums: Vec<(i32, i32)> = vec1.windows(2)
        .zip(vec2.windows(2))
        .map(|(w1, w2)| (w1.iter().sum(), w2.iter().sum()))
        .collect();
    println!("Sliding window sums: {:?}", sliding_sums);
    
    // 9. 使用cycle创建循环迭代器
    let cycled: Vec<i32> = vec1.iter()
        .cycle()
        .take(8)
        .copied()
        .collect();
    println!("Cycled vector: {:?}", cycled);
    
    // 添加测试断言
    assert_eq!(zipped, vec![(1, 4), (2, 5), (3, 6)]);
    assert_eq!(sum_pairs, vec![5, 7, 9]);
    assert_eq!(chained, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert_eq!(cycled.len(), 8);
} 