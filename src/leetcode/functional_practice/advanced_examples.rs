use std::collections::HashMap;

pub fn demonstrate_advanced_examples() {
    // 1. 数据处理流水线 - 处理学生成绩
    let grades = vec![
        ("Alice", vec![85, 90, 95, 88, 92]),
        ("Bob", vec![75, 80, 85, 90, 88]),
        ("Charlie", vec![95, 92, 88, 85, 90]),
    ];

    let analysis_results = grades.into_iter()
        .map(|(name, scores)| {
            let avg = scores.iter().sum::<i32>() as f64 / scores.len() as f64;
            let max = *scores.iter().max().unwrap_or(&0);
            let min = *scores.iter().min().unwrap_or(&0);
            let passed = scores.iter().filter(|&&x| x >= 85).count();
            (name, (avg, max, min, passed))
        })
        .collect::<HashMap<_, _>>();

    println!("成绩分析: {:?}", analysis_results);

    // 2. 文本处理 - 词频统计和分析
    let text = "rust is a great language rust is fast and reliable rust has great community";
    let word_stats = text.split_whitespace()
        .fold(HashMap::new(), |mut acc, word| {
            *acc.entry(word).or_insert(0) += 1;
            acc
        })
        .into_iter()
        .filter(|&(_, count)| count > 1)
        .collect::<HashMap<_, _>>();

    println!("重复词统计: {:?}", word_stats);

    // 3. 数据转换和验证管道
    let raw_data = vec![
        "123", "456", "abc", "789", "def", "0",
    ];

    let (numbers, non_numbers): (Vec<&str>, Vec<&str>) = raw_data.iter()
        .partition(|&&s| s.chars().all(|c| c.is_digit(10)));

    let valid_numbers: Vec<i32> = numbers.into_iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .filter(|&n| n > 100)
        .collect();

    println!("有效数字: {:?}", valid_numbers);
    println!("非数字: {:?}", non_numbers);

    // 4. 复杂数据结构转换
    let nested_data = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ];

    let transformed = nested_data.iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().map(move |(j, &val)| {
                (i, j, val)
            })
        })
        .filter(|&(_, _, val)| val % 2 == 0)
        .map(|(i, j, val)| format!("({}, {}) = {}", i, j, val))
        .collect::<Vec<_>>();

    println!("矩阵偶数位置: {:?}", transformed);

    // 5. 递归数据处理
    fn fibonacci_sequence(n: usize) -> Vec<i32> {
        std::iter::successors(Some((0, 1)), |&(a, b)| {
            Some((b, a + b))
        })
        .take(n)
        .map(|(a, _)| a)
        .collect()
    }

    let fib = fibonacci_sequence(10);
    println!("斐波那契数列: {:?}", fib);

    // 6. 组合多个数据源
    let users = vec!["user1", "user2", "user3"];
    let scores = vec![100, 200, 300];
    let active = vec![true, false, true];

    let user_data: Vec<_> = users.into_iter()
        .zip(scores.into_iter())
        .zip(active.into_iter())
        .filter(|&((_, _), active)| active)
        .map(|((user, score), _)| {
            format!("{}: {}", user, score)
        })
        .collect();

    println!("活跃用户数据: {:?}", user_data);

    // 7. 自定义迭代器
    struct WindowIterator<I: Iterator> {
        iter: I,
        window_size: usize,
        buffer: Vec<I::Item>,
    }

    impl<I> Iterator for WindowIterator<I>
    where
        I: Iterator,
        I::Item: Clone,
    {
        type Item = Vec<I::Item>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.buffer.len() < self.window_size {
                while let Some(item) = self.iter.next() {
                    self.buffer.push(item);
                    if self.buffer.len() == self.window_size {
                        break;
                    }
                }
            }

            if self.buffer.len() == self.window_size {
                let result = self.buffer.clone();
                if let Some(next_item) = self.iter.next() {
                    self.buffer.remove(0);
                    self.buffer.push(next_item);
                    Some(result)
                } else {
                    None
                }
            } else {
                None
            }
        }
    }

    let numbers = vec![1, 2, 3, 4, 5, 6];
    let window_iter = WindowIterator {
        iter: numbers.into_iter(),
        window_size: 3,
        buffer: Vec::new(),
    };

    let windowed_sums: Vec<i32> = window_iter
        .map(|window| window.iter().sum())
        .collect();

    println!("滑动窗口和: {:?}", windowed_sums);
} 