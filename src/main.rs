mod leetcode;

use leetcode::functional_practice::{
    iterator_basics,
    transform_ops,
    filter_ops,
    fold_scan_ops,
    find_ops,
    collect_ops,
    combine_ops,
    predicate_ops,
    advanced_examples
};

use leetcode::p0001_two_sum::{
    solution_traditional as two_sum_traditional,
    solution_functional as two_sum_functional,
    solution_optimized as two_sum_optimized
};

use leetcode::p0002_add_two_numbers::{
    list_node::ListNode,
    solution_traditional as add_numbers_traditional,
    solution_functional as add_numbers_functional,
    solution_recursive as add_numbers_recursive
};

use leetcode::p0003_longest_substring::{
    solution_traditional as substring_traditional,
    solution_functional as substring_functional,
    solution_optimized as substring_optimized,
};

use leetcode::p0004_median_sorted_arrays::{
    solution_traditional as median_traditional,
    solution_functional as median_functional,
    solution_optimized as median_optimized,
};

use leetcode::p0005_longest_palindrome::{
    solution_traditional as palindrome_traditional,
    solution_functional as palindrome_functional,
    solution_optimized as palindrome_optimized,
};

use leetcode::p0006_zigzag_conversion::{
    solution_traditional as zigzag_traditional,
    solution_functional as zigzag_functional,
    solution_optimized as zigzag_optimized,
};

use leetcode::p0007_reverse_integer::{
    solution_traditional as reverse_traditional,
    solution_functional as reverse_functional,
    solution_optimized as reverse_optimized,
};

use leetcode::p0008_string_to_integer::{
    solution_traditional as atoi_traditional,
    solution_functional as atoi_functional,
    solution_optimized as atoi_optimized,
};

use leetcode::p0009_palindrome_number::{
    solution_traditional as palindrome_number_traditional,
    solution_functional as palindrome_number_functional,
    solution_optimized as palindrome_number_optimized,
};

use leetcode::p0010_regular_expression::{
    solution_traditional as regex_traditional,
    solution_functional as regex_functional,
    solution_optimized as regex_optimized,
};

use leetcode::p0011_container_with_most_water::{
    solution_traditional as container_traditional,
    solution_functional as container_functional,
    solution_optimized as container_optimized,
};

use leetcode::p0012_integer_to_roman::{
    solution_traditional as roman_traditional,
    solution_functional as roman_functional,
    solution_optimized as roman_optimized,
};

use leetcode::p0013_roman_to_integer::{
    solution_traditional as roman_to_int_traditional,
    solution_functional as roman_to_int_functional,
    solution_optimized as roman_to_int_optimized,
};

use leetcode::p0014_longest_common_prefix::{
    solution_traditional as prefix_traditional,
    solution_functional as prefix_functional,
    solution_optimized as prefix_optimized,
};

use leetcode::p0015_three_sum::{
    solution_traditional as three_sum_traditional,
    solution_functional as three_sum_functional,
    solution_optimized as three_sum_optimized,
};

fn main() {
    println!("测试基本迭代器操作：");
    iterator_basics::demonstrate_iterator_basics();
    
    println!("\n测试转换操作：");
    transform_ops::demonstrate_transform_operations();
    
    println!("\n测试过滤操作：");
    filter_ops::demonstrate_filter_operations();
    
    println!("\n测试折叠和扫描操作：");
    fold_scan_ops::demonstrate_fold_scan_operations();
    
    println!("\n测试查找操作：");
    find_ops::demonstrate_find_operations();
    
    println!("\n测试收集操作：");
    collect_ops::demonstrate_collect_operations();
    
    println!("\n测试组合操作：");
    combine_ops::demonstrate_combine_operations();
    
    println!("\n测试判断操作：");
    predicate_ops::demonstrate_predicate_operations();
    
    println!("\n测试高级示例：");
    advanced_examples::demonstrate_advanced_examples();
    
    test_two_sum();
    test_add_two_numbers();
    test_longest_substring();
    test_median_sorted_arrays();
    test_longest_palindrome();
    test_zigzag_conversion();
    test_reverse_integer();
    test_string_to_integer();
    test_palindrome_number();
    test_regular_expression();
    test_container_with_most_water();
    test_integer_to_roman();
    test_roman_to_integer();
    test_longest_common_prefix();
    test_three_sum();
}

fn test_two_sum() {
    let test_cases = vec![
        (vec![2, 7, 11, 15], 9),
        (vec![3, 2, 4], 6),
        (vec![3, 3], 6),
        (vec![1, 2, 3, 4, 5], 9),
    ];

    for (nums, target) in test_cases {
        println!("\n测试用例: nums = {:?}, target = {}", nums, target);
        
        // 保存原始数组的副本用于验证
        let nums_original = nums.clone();
        
        let nums_clone = nums.clone();
        let result1 = two_sum_traditional::two_sum(nums_clone, target);
        println!("传统解法结果: {:?}", result1);
        
        let nums_clone = nums.clone();
        let result2 = two_sum_functional::two_sum(nums_clone, target);
        println!("函数式解法结果: {:?}", result2);
        
        let result3 = two_sum_optimized::two_sum(nums, target);
        println!("优化解法结果: {:?}", result3);
        
        // 使用原始数组副本进行验证
        if !result1.is_empty() {
            let i = result1[0] as usize;
            let j = result1[1] as usize;
            assert_eq!(nums_original[i] + nums_original[j], target);
        }
    }
}

fn test_add_two_numbers() {
    let test_cases = vec![
        (vec![2, 4, 3], vec![5, 6, 4]),      // 342 + 465 = 807
        (vec![0], vec![0]),                   // 0 + 0 = 0
        (vec![9,9,9,9], vec![9,9,9,9,9,9]),  // 9999 + 999999 = 1009998
    ];

    for (arr1, arr2) in test_cases {
        println!("\n测试用例: l1 = {:?}, l2 = {:?}", arr1, arr2);
        
        let l1 = ListNode::from_array(&arr1);
        let l2 = ListNode::from_array(&arr2);
        
        let result1 = add_numbers_traditional::add_two_numbers(l1.clone(), l2.clone());
        println!("传统解法结果: {:?}", ListNode::to_array(result1));
        
        let result2 = add_numbers_functional::add_two_numbers(l1.clone(), l2.clone());
        println!("函数式解法结果: {:?}", ListNode::to_array(result2));
        
        let result3 = add_numbers_recursive::add_two_numbers(l1, l2);
        println!("递归解法结果: {:?}", ListNode::to_array(result3));
    }
}

fn test_longest_substring() {
    let test_cases = vec![
        "abcabcbb",    // 3 ("abc")
        "bbbbb",       // 1 ("b")
        "pwwkew",      // 3 ("wke")
        "",           // 0
        "dvdf",       // 3 ("vdf")
        "abba",       // 2 ("ab" or "ba")
    ];

    for s in test_cases {
        println!("\n测试用例: s = \"{}\"", s);
        
        let result1 = substring_traditional::length_of_longest_substring(s.to_string());
        println!("传统解法结果: {}", result1);
        
        let result2 = substring_functional::length_of_longest_substring(s.to_string());
        println!("函数式解法结果: {}", result2);
        
        let result3 = substring_optimized::length_of_longest_substring(s.to_string());
        println!("优化解法结果: {}", result3);
        
        // 验证三种解法结果一致
        assert_eq!(result1, result2);
        assert_eq!(result2, result3);
    }
}

fn test_median_sorted_arrays() {
    let test_cases = vec![
        (vec![1, 3], vec![2]),                    // 2.0
        (vec![1, 2], vec![3, 4]),                 // 2.5
        (vec![], vec![1]),                        // 1.0
        (vec![2], vec![]),                        // 2.0
        (vec![1, 3, 5], vec![2, 4, 6]),          // 3.5
        (vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9]), // 5.0
    ];

    for (nums1, nums2) in test_cases {
        println!("\n测试用例: nums1 = {:?}, nums2 = {:?}", nums1, nums2);
        
        let result1 = median_traditional::find_median_sorted_arrays(nums1.clone(), nums2.clone());
        println!("传统解法结果: {}", result1);
        
        let result2 = median_functional::find_median_sorted_arrays(nums1.clone(), nums2.clone());
        println!("函数式解法结果: {}", result2);
        
        let result3 = median_optimized::find_median_sorted_arrays(nums1, nums2);
        println!("优化解法结果: {}", result3);
        
        // 验证三种解法结果一致
        assert!((result1 - result2).abs() < 1e-10);
        assert!((result2 - result3).abs() < 1e-10);
    }
}

fn test_longest_palindrome() {
    let test_cases = vec![
        "babad",           // "bab" or "aba"
        "cbbd",            // "bb"
        "a",              // "a"
        "ac",             // "a" or "c"
        "aacabdkacaa",    // "aca"
        "abcba",          // "abcba"
    ];

    for s in test_cases {
        println!("\n测试用例: s = \"{}\"", s);
        
        let result1 = palindrome_traditional::longest_palindrome(s.to_string());
        println!("传统解法结果: {}", result1);
        
        let result2 = palindrome_functional::longest_palindrome(s.to_string());
        println!("函数式解法结果: {}", result2);
        
        let result3 = palindrome_optimized::longest_palindrome(s.to_string());
        println!("优化解法结果: {}", result3);
        
        // 验证结果都是回文串
        assert!(is_palindrome(&result1));
        assert!(is_palindrome(&result2));
        assert!(is_palindrome(&result3));
        // 验证长度相同（可能有多个最长回文子串）
        assert_eq!(result1.len(), result2.len());
        assert_eq!(result2.len(), result3.len());
    }
}

fn is_palindrome(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    for i in 0..len/2 {
        if chars[i] != chars[len-1-i] {
            return false;
        }
    }
    true
}

fn test_zigzag_conversion() {
    let test_cases = vec![
        ("PAYPALISHIRING", 3),  // "PAHNAPLSIIGYIR"
        ("PAYPALISHIRING", 4),  // "PINALSIGYAHRPI"
        ("A", 1),              // "A"
        ("AB", 1),             // "AB"
        ("ABC", 2),            // "ACB"
        ("ABCDE", 4),          // "ABCED"
    ];

    for (s, num_rows) in test_cases {
        println!("\n测试用例: s = \"{}\", num_rows = {}", s, num_rows);
        
        let result1 = zigzag_traditional::convert(s.to_string(), num_rows);
        println!("传统解法结果: {}", result1);
        
        let result2 = zigzag_functional::convert(s.to_string(), num_rows);
        println!("函数式解法结果: {}", result2);
        
        let result3 = zigzag_optimized::convert(s.to_string(), num_rows);
        println!("优化解法结果: {}", result3);
        
        // 验证三种解法结果一致
        assert_eq!(result1, result2);
        assert_eq!(result2, result3);
    }
}

fn test_reverse_integer() {
    let test_cases: Vec<i32> = vec![
        123,        // 321
        -123,       // -321
        120,        // 21
        0,          // 0
        1534236469, // 0 (overflow)
        -2147483648,// 0 (overflow)
    ];

    for x in test_cases {
        println!("\n测试用例: x = {}", x);
        
        let result1 = reverse_traditional::reverse(x);
        println!("传统解法结果: {}", result1);
        
        let result2 = reverse_functional::reverse(x);
        println!("函数式解法结果: {}", result2);
        
        let result3 = reverse_optimized::reverse(x);
        println!("优化解法结果: {}", result3);
        
        // 验证三种解法结果一致
        assert_eq!(result1, result2);
        assert_eq!(result2, result3);
        
        // 验证结果正确性
        if result1 != 0 {
            // 安全地处理数字转换，避免溢出
            let abs_str = if x == i32::MIN {
                String::from("2147483648")
            } else {
                let abs_num = x.abs();
                abs_num.to_string()
            };
            
            let reversed_str = abs_str.chars().rev().collect::<String>();
            let expected = reversed_str.parse::<i32>()
                .map(|n| if x < 0 { -n } else { n })
                .unwrap_or(0);
            
            assert_eq!(result1, expected);
        }
    }
}

fn test_string_to_integer() {
    let test_cases = vec![
        "42",               // 42
        "   -42",          // -42
        "4193 with words", // 4193
        "words and 987",   // 0
        "-91283472332",    // -2147483648
        "3.14159",         // 3
        "+1",              // 1
        "+-12",            // 0
        "",                // 0
        "  ",              // 0
    ];

    for s in test_cases {
        println!("\n测试用例: s = \"{}\"", s);
        
        let result1 = atoi_traditional::my_atoi(s.to_string());
        println!("传统解法结果: {}", result1);
        
        let result2 = atoi_functional::my_atoi(s.to_string());
        println!("函数式解法结果: {}", result2);
        
        let result3 = atoi_optimized::my_atoi(s.to_string());
        println!("优化解法结果: {}", result3);
        
        // 验证三种解法结果一致
        assert_eq!(result1, result2);
        assert_eq!(result2, result3);
    }
}

fn test_palindrome_number() {
    let test_cases = vec![
        121,        // true
        -121,       // false
        10,         // false
        0,          // true
        12321,      // true
        12345,      // false
        1000021,    // false
    ];

    for x in test_cases {
        println!("\n测试用例: x = {}", x);
        
        let result1 = palindrome_number_traditional::is_palindrome(x);
        println!("传统解法结果: {}", result1);
        
        let result2 = palindrome_number_functional::is_palindrome(x);
        println!("函数式解法结果: {}", result2);
        
        let result3 = palindrome_number_optimized::is_palindrome(x);
        println!("优化解法结果: {}", result3);
        
        // 验证三种解法结果一致
        assert_eq!(result1, result2);
        assert_eq!(result2, result3);
    }
}

fn test_regular_expression() {
    let test_cases = vec![
        ("aa", "a"),             // false
        ("aa", "a*"),            // true
        ("ab", ".*"),            // true
        ("mississippi", "mis*is*p*."), // false
        ("aab", "c*a*b"),        // true
        ("", ".*"),              // true
        ("a", "ab*"),            // true
        ("aaa", "a*a"),          // true
    ];

    for (s, p) in test_cases {
        println!("\n测试用例: s = \"{}\", p = \"{}\"", s, p);
        
        let result1 = regex_traditional::is_match(s.to_string(), p.to_string());
        println!("传统解法结果: {}", result1);
        
        let result2 = regex_functional::is_match(s.to_string(), p.to_string());
        println!("函数式解法结果: {}", result2);
        
        let result3 = regex_optimized::is_match(s.to_string(), p.to_string());
        println!("优化解法结果: {}", result3);
        
        // 验证三种解法结果一致
        assert_eq!(result1, result2);
        assert_eq!(result2, result3);
    }
}

fn test_container_with_most_water() {
    let test_cases = vec![
        vec![1,8,6,2,5,4,8,3,7],  // 49
        vec![1,1],                 // 1
        vec![4,3,2,1,4],          // 16
        vec![1,2,1],              // 2
        vec![1,8,6,2,5,4,8,25,7], // 49
    ];

    for height in test_cases {
        println!("\n测试用例: height = {:?}", height);
        
        let result1 = container_traditional::max_area(height.clone());
        println!("传统解法结果: {}", result1);
        
        let result2 = container_functional::max_area(height.clone());
        println!("函数式解法结果: {}", result2);
        
        let result3 = container_optimized::max_area(height);
        println!("优化解法结果: {}", result3);
        
        // 验证三种解法结果一致
        assert_eq!(result1, result2);
        assert_eq!(result2, result3);
    }
}

fn test_integer_to_roman() {
    let test_cases = vec![
        3,      // "III"
        4,      // "IV"
        9,      // "IX"
        58,     // "LVIII"
        1994,   // "MCMXCIV"
        3999,   // "MMMCMXCIX"
    ];

    for num in test_cases {
        println!("\n测试用例: num = {}", num);
        
        let result1 = roman_traditional::int_to_roman(num);
        println!("传统解法结果: {}", result1);
        
        let result2 = roman_functional::int_to_roman(num);
        println!("函数式解法结果: {}", result2);
        
        let result3 = roman_optimized::int_to_roman(num);
        println!("优化解法结果: {}", result3);
        
        // 验证三种解法结果一致
        assert_eq!(result1, result2);
        assert_eq!(result2, result3);
    }
}

fn test_roman_to_integer() {
    let test_cases = vec![
        "III",       // 3
        "IV",        // 4
        "IX",        // 9
        "LVIII",     // 58
        "MCMXCIV",   // 1994
        "MMMCMXCIX", // 3999
    ];

    for s in test_cases {
        println!("\n测试用例: s = \"{}\"", s);
        
        let result1 = roman_to_int_traditional::roman_to_int(s.to_string());
        println!("传统解法结果: {}", result1);
        
        let result2 = roman_to_int_functional::roman_to_int(s.to_string());
        println!("函数式解法结果: {}", result2);
        
        let result3 = roman_to_int_optimized::roman_to_int(s.to_string());
        println!("优化解法结果: {}", result3);
        
        // 验证三种解法结果一致
        assert_eq!(result1, result2);
        assert_eq!(result2, result3);
    }
}

fn test_longest_common_prefix() {
    let test_cases = vec![
        vec!["flower","flow","flight"],           // "fl"
        vec!["dog","racecar","car"],             // ""
        vec!["interspecies","interstellar","interstate"],  // "inters"
        vec!["throne","throne"],                  // "throne"
        vec![""],                                 // ""
        vec!["a"],                                // "a"
        vec!["","b"],                             // ""
    ];

    for strs in test_cases {
        let strs: Vec<String> = strs.into_iter().map(String::from).collect();
        println!("\n测试用例: strs = {:?}", strs);
        
        let result1 = prefix_traditional::longest_common_prefix(strs.clone());
        println!("传统解法结果: {}", result1);
        
        let result2 = prefix_functional::longest_common_prefix(strs.clone());
        println!("函数式解法结果: {}", result2);
        
        let result3 = prefix_optimized::longest_common_prefix(strs);
        println!("优化解法结果: {}", result3);
        
        // 验证三种解法结果一致
        assert_eq!(result1, result2);
        assert_eq!(result2, result3);
    }
}

fn test_three_sum() {
    let test_cases = vec![
        vec![-1,0,1,2,-1,-4],              // [[-1,-1,2],[-1,0,1]]
        vec![],                            // []
        vec![0],                           // []
        vec![0,0,0],                       // [[0,0,0]]
        vec![-2,0,1,1,2],                 // [[-2,0,2],[-2,1,1]]
        vec![-1,0,1,2,-1,-4,-2,-3,3,0,4], // 多个解
    ];

    for nums in test_cases {
        println!("\n测试用例: nums = {:?}", nums);
        
        let mut result1 = three_sum_traditional::three_sum(nums.clone());
        let mut result2 = three_sum_functional::three_sum(nums.clone());
        let mut result3 = three_sum_optimized::three_sum(nums);
        
        // 排序结果以便比较
        result1.sort();
        result2.sort();
        result3.sort();
        
        println!("传统解法结果: {:?}", result1);
        println!("函数式解法结果: {:?}", result2);
        println!("优化解法结果: {:?}", result3);
        
        // 验证三种解法结果一致
        assert_eq!(result1, result2);
        assert_eq!(result2, result3);
    }
}
