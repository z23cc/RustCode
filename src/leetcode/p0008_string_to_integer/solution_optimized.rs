#[derive(Clone, Copy)]
enum State {
    Start,
    Sign,
    Number,
    End,
}

pub fn my_atoi(s: String) -> i32 {
    let mut state = State::Start;
    let mut result: i64 = 0;
    let mut sign = 1;
    
    for c in s.chars() {
        match state {
            State::Start => {
                match c {
                    ' ' => continue,
                    '+' => state = State::Sign,
                    '-' => {
                        sign = -1;
                        state = State::Sign;
                    },
                    '0'..='9' => {
                        result = (c as u8 - b'0') as i64;
                        state = State::Number;
                    },
                    _ => state = State::End,
                }
            },
            State::Sign => {
                match c {
                    '0'..='9' => {
                        result = (c as u8 - b'0') as i64;
                        state = State::Number;
                    },
                    _ => state = State::End,
                }
            },
            State::Number => {
                match c {
                    '0'..='9' => {
                        result = result * 10 + (c as u8 - b'0') as i64;
                        if result * sign > i32::MAX as i64 {
                            return i32::MAX;
                        }
                        if result * sign < i32::MIN as i64 {
                            return i32::MIN;
                        }
                    },
                    _ => state = State::End,
                }
            },
            State::End => break,
        }
    }
    
    (result * sign) as i32
} 