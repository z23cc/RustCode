pub fn demonstrate_find_operations() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // 1. find - 查找第一个满足条件的元素
    let first_even = numbers.iter()
        .find(|&&x| x % 2 == 0);
    println!("First even number: {:?}", first_even);
    
    // 2. position - 查找元素的位置
    let position_of_five = numbers.iter()
        .position(|&x| x == 5);
    println!("Position of 5: {:?}", position_of_five);
    
    // 3. rposition - 从右边开始查找位置
    let last_even_position = numbers.iter()
        .rposition(|&x| x % 2 == 0);
    println!("Position of last even number: {:?}", last_even_position);
    
    // 4. find_map - 查找并转换
    let first_even_squared = numbers.iter()
        .find_map(|&x| {
            if x % 2 == 0 {
                Some(x * x)
            } else {
                None
            }
        });
    println!("First even number squared: {:?}", first_even_squared);
    
    // 5. 组合查找和其他操作
    let first_big_even = numbers.iter()
        .filter(|&&x| x > 5)
        .find(|&&x| x % 2 == 0);
    println!("First even number greater than 5: {:?}", first_big_even);
    
    // 6. any - 检查是否存在满足条件的元素
    let has_even = numbers.iter().any(|&x| x % 2 == 0);
    let has_negative = numbers.iter().any(|&x| x < 0);
    println!("Has even numbers: {}", has_even);
    println!("Has negative numbers: {}", has_negative);
    
    // 7. all - 检查是否所有元素都满足条件
    let all_positive = numbers.iter().all(|&x| x > 0);
    let all_even = numbers.iter().all(|&x| x % 2 == 0);
    println!("All numbers are positive: {}", all_positive);
    println!("All numbers are even: {}", all_even);
    
    // 8. 复杂查找示例 - 查找连续的偶数
    let consecutive_evens = numbers.windows(2)
        .find(|pair| pair[0] % 2 == 0 && pair[1] % 2 == 0)
        .map(|pair| (pair[0], pair[1]));
    println!("First consecutive even numbers: {:?}", consecutive_evens);
    
    // 9. 自定义查找逻辑
    let sum_to_ten = numbers.iter()
        .scan(0, |sum, &x| {
            *sum += x;
            Some(*sum)
        })
        .find(|&sum| sum == 10);
    println!("First sum that equals 10: {:?}", sum_to_ten);
} 