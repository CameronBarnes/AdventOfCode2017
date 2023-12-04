use std::collections::HashSet;

fn get_sorted_word(str: &str) -> String {
    let mut vec: Vec<char> = str.chars().collect();
    vec.sort_unstable();
    String::from_iter(vec)
}

fn check_no_anagram(str: &str) -> bool {

    let mut set: HashSet<String> = HashSet::new();
    let mut counter = 0;
    str.split_whitespace().for_each(|word| {
        counter += 1;
        set.insert(get_sorted_word(word));
    });

    set.len() == counter

}

#[tracing::instrument]
pub fn process(input: &str) -> String {
        input.lines().filter(|line| check_no_anagram(line)).count().to_string()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "abcde fghij
abcde xyz ecdab
a ab abc abd abf abj
iiii oiii ooii oooi oooo
oiii ioii iioi iiio";
        assert_eq!("3", process(input));
    }
}
