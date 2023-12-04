use std::collections::HashSet;

fn as_str(bank: &[usize]) -> String {
    let mut str = String::new();
    for i in bank {
        str.push_str(&i.to_string());
        str.push(' ');
    }
    str
}

fn get_biggest(bank: &[usize]) -> usize {
    let mut biggest = 0;
    (0..bank.len()).for_each(|i| {
        if bank[i] > bank[biggest] {
            biggest = i;
        }
    });
    biggest
}

fn reallocate(bank: &mut Vec<usize>) {

    let biggest = get_biggest(bank);

    let mut index = biggest;
    let mut num = bank[biggest];
    bank[biggest] = 0;

    while num > 0 {

        index += 1;
        if index >= bank.len() {
            index = 0;
        }
        bank[index] += 1;
        num -= 1;

    }

}

#[tracing::instrument]
pub fn process(input: &str) -> String {

    let mut banks: Vec<usize> = input.split_whitespace().flat_map(str::parse::<usize>).collect();
    let mut cycle = 0;
    let mut set: HashSet<String> = HashSet::new();
    set.insert(as_str(&banks));
    let mut cont = true;

    while cont {
        cycle += 1;
        reallocate(&mut banks);
        let str = as_str(&banks);
        if set.contains(&str) {
            cont = false;
        } else {
            set.insert(str);
        }
    }

    cycle.to_string()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "0 2 7 0";
        assert_eq!("5", process(input));
    }
}
