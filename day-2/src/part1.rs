
#[tracing::instrument]
pub fn process(input: &str) -> String {

    input.lines().map(str::trim).map(|line| {
        let (min, max) = line.split_whitespace().fold((99999999, 0), |(min, max), x| {
            //println!("{}", x);
            let x = x.parse().expect("Will always be a number as we're spliting out the whitespace");
            (i32::min(min, x), i32::max(max, x))
        });
        max - min
    }).sum::<i32>().to_string()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "5 1 9 5
7 5 3
2 4 6 8";
        assert_eq!("18", process(input));
    }
}
