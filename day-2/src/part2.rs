
#[tracing::instrument]
pub fn process(input: &str) -> String {
    input.lines().map(str::trim).map(|line| {
        let nums: Vec<i32> = line.split_whitespace().flat_map(str::parse::<i32>).collect();
        for x in 0..nums.len() {
            for y in 0..nums.len() {
                if x == y {
                    continue;
                } else if nums[x] % nums[y] == 0 {
                    return nums[x] / nums[y];
                }
            }
        }
        0
    }).sum::<i32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "5 9 2 8
9 4 7 3
3 8 6 5";
        assert_eq!("9", process(input));
    }
}
