pub fn reverse(input: &str) -> String {
    //we can use the following library for the bonus task- https://crates.io/crates/unicode-reverse
    input.chars().rev().collect()
}
