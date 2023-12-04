
#[tracing::instrument]
pub fn process(input: &str) -> String {
    let mut sum = 0;
    let chars: Vec<char> = input.trim().chars().collect();
    for i in 0..chars.len() {
        let mut next = i + chars.len() / 2;
        if next >= chars.len() {
            next -= chars.len();
        }

        if chars[i] == chars[next] {
            sum += chars[i].to_digit(10).expect("Input only contains numbers");
        }
    }
    
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!("6", process("1212"));
        assert_eq!("0", process("1221"));
        assert_eq!("4", process("123425"));
        assert_eq!("12", process("123123"));
        assert_eq!("4", process("12131415"));
    }
}
