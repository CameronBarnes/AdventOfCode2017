
#[tracing::instrument]
pub fn process(input: &str) -> String {
    
    let mut instructions: Vec<i32> = input.lines().flat_map(|line| line.trim().parse::<i32>()).collect();
    let mut steps = 0;
    let mut index = 0;

    while let Some(value) = instructions.get_mut(index) {
        steps += 1;
        index = i32::max(0, index as i32 + *value) as usize;
        *value += 1;
    }

    steps.to_string()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "0
3
0
1
-3";
        assert_eq!("5", process(input));
    }
}
