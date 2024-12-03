pub fn int_to_roman(num: i32) -> String {
    let conversions = [
        (1000, "M"), (900, "CM"),
        (500, "D"), (400, "CD"),
        (100, "C"), (90, "XC"),
        (50, "L"), (40, "XL"),
        (10, "X"), (9, "IX"),
        (5, "V"), (4, "IV"),
        (1, "I")
    ];
    
    conversions.iter()
        .fold((num, String::new()), |(n, acc), &(val, sym)| {
            let count = n / val;
            let remainder = n % val;
            (remainder, acc + &sym.repeat(count as usize))
        })
        .1
} 