/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if code.len() <= 1 {
        return false;
    }

    // Check if a non numeric and non space char is present
    if code.chars().filter(|c| (!c.is_numeric() || !c.is_ascii()) && !c.is_whitespace()).collect::<String>().len() > 0 {
        println!("Invalid Char");
        return false
    }

    let stripped = code.chars().filter(|c| c.is_numeric());
    let stripped = stripped.rev().enumerate().collect::<Vec<_>>();

    if stripped.len() <= 1 {
        return false
    }


    let mut numbers: Vec<u32> = Vec::new();

    for (i, v) in stripped {
        let mut v = v.to_digit(10).unwrap();
        println!("V is {}", v);
        if i % 2 != 0 {
            v *= 2;
            if v > 9 {
                v -= 9;
            }
            println!("Even V is now {}", v);     
        }
        numbers.push(v);
    }

    println!("Numbers: {:?}", numbers);
    let sum: u32 = numbers.iter().sum();
    println!("Sum: {}", sum);
    sum % 10 == 0
}
