struct Prgm {
    name: String,
    weight: usize,

}

fn parse_line(str: &str) -> Prgm {

}

#[tracing::instrument]
pub fn process(_input: &str) -> String {
    todo!("day-7 - part 1")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";
        assert_eq!("tknk", process(input));
    }
}
