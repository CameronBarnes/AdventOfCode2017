use std::collections::HashSet;

fn check_no_duplicate(str: &str) -> bool {

    let mut set: HashSet<&str> = HashSet::new();
    let mut counter = 0;
    str.split_whitespace().for_each(|word| {
        counter += 1;
        set.insert(word);
    });

    set.len() == counter

}

#[tracing::instrument]
pub fn process(input: &str) -> String {
    input.lines().filter(|line| check_no_duplicate(line)).count().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "aa bb cc dd ee
aa bb cc dd aa
aa bb cc dd aaa";
        assert_eq!("2", process(input));
    }
}
