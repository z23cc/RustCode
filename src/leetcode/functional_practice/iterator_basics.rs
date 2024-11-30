pub fn demonstrate_iterator_basics() {
    // 1. 创建迭代器的基本方法
    let vec = vec![1, 2, 3, 4, 5];
    
    // iter() - 创建不可变引用的迭代器
    let _iter = vec.iter();
    
    // iter_mut() - 创建可变引用的迭代器
    let mut vec_mut = vec.clone();
    let _iter_mut = vec_mut.iter_mut();
    
    // into_iter() - 获取所有权的迭代器
    let _iter_owned = vec.clone().into_iter();
    
    // 2. 基本迭代器方法
    // next() - 获取下一个元素
    let mut iter = vec.iter();
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    
    // count() - 计算剩余元素数量
    assert_eq!(iter.count(), 3); // 3,4,5 还未被消费
    
    // last() - 获取最后一个元素
    let mut iter = vec.iter();
    assert_eq!(iter.last(), Some(&5));
    
    // nth() - 获取第n个元素
    let mut iter = vec.iter();
    assert_eq!(iter.nth(2), Some(&3));
    
    // 3. 迭代器适配器
    // rev() - 反转迭代器
    let _rev_iter = vec.iter().rev();
    
    // step_by() - 设置步长
    let _step_iter = vec.iter().step_by(2); // [1, 3, 5]
    
    // chain() - 连接两个迭代器
    let vec2 = vec![6, 7, 8];
    let _chain_iter = vec.iter().chain(vec2.iter());
    
    // enumerate() - 添加索引
    let _enum_iter = vec.iter().enumerate(); // [(0, 1), (1, 2), ...]
    
    // 4. 实用方法
    println!("基本迭代器演示：");
    
    // for循环遍历
    for x in vec.iter() {
        print!("{} ", x);
    }
    println!();
    
    // 使用enumerate获取索引
    for (i, x) in vec.iter().enumerate() {
        print!("{}:{} ", i, x);
    }
    println!();
} 