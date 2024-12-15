pub fn is_valid(s: String) -> bool {
    s.chars().try_fold(Vec::new(), |mut stack, c| {
        match c {
            '(' | '[' | '{' => {
                stack.push(c);
                Some(stack)
            },
            ')' => if stack.pop() == Some('(') { Some(stack) } else { None },
            ']' => if stack.pop() == Some('[') { Some(stack) } else { None },
            '}' => if stack.pop() == Some('{') { Some(stack) } else { None },
            _ => None,
        }
    }).map_or(false, |stack| stack.is_empty())
} 