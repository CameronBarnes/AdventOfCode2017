
#[tracing::instrument]
pub fn process(input: &str) -> String {

    let mut sum = 0;
    let chars: Vec<char> = input.trim().chars().collect();
    for i in 0..chars.len() {
        let mut next = i + 1;
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
        assert_eq!("3", process("1122"));
        assert_eq!("4", process("1111"));
        assert_eq!("0", process("1234"));
        assert_eq!("9", process("91212129"));
    }
}
