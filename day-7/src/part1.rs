use std::{collections::HashMap, rc::Rc, cell::Cell};


struct PrgmHolder {
    prgm: Option<Prgm>,
    depth: usize,
}

struct Prgm {
    name: String,
    weight: usize,
    children: Vec<Rc<Cell<PrgmHolder>>>,
}

fn parse_line(str: &str) -> (String, usize, Vec<String>) {

    let mut children: Vec<String> = Vec::new();
    let first_half = if str.contains("->") {

        let mut iter = str.split("->");
        let first_half = iter.next().expect("Checked for delimiter already");
        let second_half = iter.last().expect("Checked for delimiter already");

        children.extend(second_half.trim().split(", ").map(str::to_owned));

        first_half

    } else {
        str
    };

    let mut iter = first_half.split(" (");
    let name = iter.next().expect("All lines should contain this").to_owned();
    let weight = iter.last().expect("All lines should contain this")
        .split(')').next().unwrap().parse().expect("Will always be a number");

    (name, weight, children)
}

#[tracing::instrument]
pub fn process(input: &str) -> String {

    let mut map: HashMap<String, Rc<Cell<PrgmHolder>>> = HashMap::new();
    input.lines().map(parse_line).for_each(|(name, weight, children)| {

        let children = children.iter().map(|child| {
            if let Some(val) = map.get(child) {
                val.clone()
            } else {
                Rc::new(Cell::new(PrgmHolder{prgm: None, depth: 1}))
            }
        }).collect();

        if map.contains_key(&name) {
            map.get_mut(&name).unwrap().get_mut().prgm = Some(Prgm{name: name.clone(), weight, children});
        } else {
            map.insert(name.clone(), 
                       Rc::new(Cell::new(PrgmHolder{prgm: Some(Prgm{name, weight, children}), depth: 0})));
        }

    });

    let bottom = map.into_values().find(|holder| holder.clone().get_mut().depth == 0)
        .unwrap().clone().get_mut().prgm.unwrap();
    bottom.name

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
