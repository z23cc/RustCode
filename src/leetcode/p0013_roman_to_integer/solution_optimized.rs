pub fn roman_to_int(s: String) -> i32 {
    let mut result = 0;
    let bytes = s.as_bytes();
    let mut i = 0;
    
    while i < bytes.len() {
        match bytes[i] {
            b'M' => result += 1000,
            b'D' => result += 500,
            b'C' => {
                if i + 1 < bytes.len() {
                    match bytes[i + 1] {
                        b'M' => { result += 900; i += 1; }
                        b'D' => { result += 400; i += 1; }
                        _ => result += 100,
                    }
                } else {
                    result += 100;
                }
            }
            b'L' => result += 50,
            b'X' => {
                if i + 1 < bytes.len() {
                    match bytes[i + 1] {
                        b'C' => { result += 90; i += 1; }
                        b'L' => { result += 40; i += 1; }
                        _ => result += 10,
                    }
                } else {
                    result += 10;
                }
            }
            b'V' => result += 5,
            b'I' => {
                if i + 1 < bytes.len() {
                    match bytes[i + 1] {
                        b'X' => { result += 9; i += 1; }
                        b'V' => { result += 4; i += 1; }
                        _ => result += 1,
                    }
                } else {
                    result += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }
    
    result
} 