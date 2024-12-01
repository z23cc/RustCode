pub fn is_palindrome(x: i32) -> bool {
    // 负数不是回文数
    if x < 0 {
        return false;
    }
    
    // 0 是回文数
    if x == 0 {
        return true;
    }
    
    // 末尾为0的非0数不是回文数
    if x % 10 == 0 {
        return false;
    }
    
    let mut num = x;
    let mut reversed = 0;
    
    // 只反转一半数字
    while num > reversed {
        reversed = reversed * 10 + num % 10;
        num /= 10;
    }
    
    // 当数字长度为奇数时，需要去掉中间的数字
    num == reversed || num == reversed / 10
} 