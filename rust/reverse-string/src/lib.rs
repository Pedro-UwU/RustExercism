pub fn reverse(input: &str) -> String {
    let src = String::from(input);
    src.chars().rev().collect()
}
