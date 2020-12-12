/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code_no_space:String = code.split_ascii_whitespace().collect();
    if code_no_space.chars().all(char::is_numeric){
        if code_no_space.len() <= 1 {
            return false
        }
        let mut even = false;
        let mut total_sum = 0;
        for ch in code_no_space.chars().rev(){
            let mut digit = ch as u32 - '0' as u32;
            if even && digit != 9{
                digit = digit * 2 % 9;
            }
            total_sum += digit;
            even = !even;
        }
        return total_sum % 10 == 0
    }
    false
}
