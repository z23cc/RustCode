use std::collections::{HashMap, HashSet, BTreeSet, VecDeque};

pub fn demonstrate_collect_operations() {
    let numbers = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    
    // 1. 收集到 Vec
    let vec_collected: Vec<i32> = numbers.iter()
        .map(|&x| x * 2)
        .collect();
    println!("Collected to Vec: {:?}", vec_collected);
    
    // 2. 收集到 HashSet (去重)
    let set_collected: HashSet<i32> = numbers.iter()
        .copied()
        .collect();
    println!("Collected to HashSet: {:?}", set_collected);
    
    // 3. 收集到 BTreeSet (排序去重)
    let btree_set: BTreeSet<i32> = numbers.iter()
        .copied()
        .collect();
    println!("Collected to BTreeSet: {:?}", btree_set);
    
    // 4. 收集到 VecDeque
    let deque: VecDeque<i32> = numbers.iter()
        .copied()
        .collect();
    println!("Collected to VecDeque: {:?}", deque);
    
    // 5. 收集到 HashMap
    let map: HashMap<i32, i32> = numbers.iter()
        .enumerate()
        .map(|(i, &v)| (v, i as i32))
        .collect();
    println!("Collected to HashMap: {:?}", map);
    
    // 6. partition - 分割成两个集合
    let (evens, odds): (Vec<i32>, Vec<i32>) = numbers.iter()
        .copied()
        .partition(|x| x % 2 == 0);
    println!("Partitioned to evens: {:?}", evens);
    println!("Partitioned to odds: {:?}", odds);
    
    // 7. 分组收集
    let grouped: HashMap<i32, Vec<i32>> = numbers.iter()
        .copied()
        .fold(HashMap::new(), |mut map, x| {
            map.entry(x).or_default().push(x);
            map
        });
    println!("Grouped by value: {:?}", grouped);
    
    // 8. 字符串收集
    let chars = ['h', 'e', 'l', 'l', 'o'];
    let string: String = chars.iter().collect();
    println!("Collected to String: {}", string);
    
    // 9. 复杂收集示例 - 统计频率
    let frequency: HashMap<i32, usize> = numbers.iter()
        .copied()
        .fold(HashMap::new(), |mut map, x| {
            *map.entry(x).or_default() += 1;
            map
        });
    println!("Frequency count: {:?}", frequency);
} 