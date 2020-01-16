use voca_rs::*;
use std::collections::HashMap;

type Chemical = (usize, String);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Reaction {
    from: Vec<Chemical>,
    to: Chemical,
}

impl Reaction {
    pub fn new(from: &Vec<Chemical>, to: &Chemical) -> Reaction {
        Reaction {
            from: from.clone(),
            to: to.clone(),
        }
    }

    pub fn from(&self) -> &Vec<Chemical> { &self.from }
    pub fn to(&self) -> &Chemical { &self.to }

}

/*
 10 ORE => 10 A
 1 ORE => 1 B
 7 A, 1 B => 1 C
 7 A, 1 C => 1 D
 7 A, 1 D => 1 E
 7 A, 1 E => 1 FUEL

 1 FUEL ____
 |          |
 7 A        1 E ____
 |          |       |
 10 A       7 A     1 D ____
 |          |       |       |
 10 ORE     10 A    7 A     1 C ____
            |       |       |       |
            10 ORE  10 A    7 A     1 B
                    |       |       |
                    10 ORE  10 A    1 B
                            |       |
                            10 ORE  1 ORE

 10 A = 10 ORE -> 1 A = 1 ORE
 1 B = 1 ORE  -> 1 B = 1 ORE

 1 FUEL ____
 |          |
 7 ORE(A)   1 E _____
            |        |
            7 ORE(A) 1 D _____
                     |        |
                     7 ORE(A) 1 C _____
                              |        |
                              7 ORE(A) 1 ORE(B)
 (7 + 7 + 7 + 7) + 10 - ((7 + 7 + 7 + 7) % 10) = 30
 +
 (1) + 1 - ((1) % 1) = 1
*/

#[derive(Debug, Clone)]
pub struct ChemNode {
    chem: Chemical,
    from: Option<String>,
    mul: f64,
    children: Vec<ChemNode>
}

impl ChemNode {
    pub fn new(chem: Chemical, from: Option<String>, mul: f64, children: Vec<ChemNode>) -> ChemNode {
        ChemNode {
            chem,
            from,
            mul,
            children,
        }
    }
    pub fn chem(&self) -> &Chemical { &self.chem }
    pub fn final_chem(&self) -> (f64, &String) { (self.mul * self.chem.0 as f64, &self.chem.1) }
    pub fn from(&self) -> &Option<String> { &self.from }
    pub fn mul(&self) -> f64 { self.mul }
    pub fn children(&self) -> &Vec<ChemNode> { &self.children }
}

/// ```
/// use aoc_2019::day14::{to_chem_tree, Reaction, OrePriceList};
///
/// let reactions = vec![
///     Reaction::new(&vec![(10, "ORE".to_string())], &(10, "A".to_string())),
///     Reaction::new(&vec![(1, "ORE".to_string())], &(1, "B".to_string())),
///     Reaction::new(&vec![(7, "A".to_string()), (1, "B".to_string())], &(1, "C".to_string())),
///     Reaction::new(&vec![(7, "A".to_string()), (1, "C".to_string())], &(1, "D".to_string())),
///     Reaction::new(&vec![(7, "A".to_string()), (1, "D".to_string())], &(1, "E".to_string())),
///     Reaction::new(&vec![(7, "A".to_string()), (1, "E".to_string())], &(1, "FUEL".to_string())),
/// ];
///
/// let price_list = OrePriceList::new(&reactions);
///
/// let node = to_chem_tree(
///     &reactions,
///     5,
///     &price_list,
///     None
/// );
///
/// assert_eq!(node.final_chem(), (1.0, &"FUEL".to_string()));
///
/// assert_eq!(node.children()[0].final_chem(), (7.0, &"ORE".to_string()));
/// assert_eq!(node.children()[1].final_chem(), (1.0, &"E".to_string()));
///
/// assert_eq!(node.children()[1].children()[0].final_chem(), (7.0, &"ORE".to_string()));
/// assert_eq!(node.children()[1].children()[1].final_chem(), (1.0, &"D".to_string()));
///
/// assert_eq!(node.children()[1].children()[1].children()[0].final_chem(), (7.0, &"ORE".to_string()));
/// assert_eq!(node.children()[1].children()[1].children()[1].final_chem(), (1.0, &"C".to_string()));
///
/// assert_eq!(node.children()[1].children()[1].children()[1].children()[0].final_chem(), (7.0, &"ORE".to_string()));
/// assert_eq!(node.children()[1].children()[1].children()[1].children()[1].final_chem(), (1.0, &"ORE".to_string()));
/// ```
pub fn to_chem_tree(reactions: &Vec<Reaction>, i: usize, price_list: &OrePriceList, required: Option<usize>) -> ChemNode {
    let reac = &reactions[i];
    let chem = reac.to().clone();
    if let Some(&mul) = price_list.get_cost(&chem.1) {
        ChemNode {
            chem: (required.unwrap(), "ORE".to_string()),
            from: Some(chem.1.clone()),
            mul,
            children: vec![],
        }
    } else {
        let mut children = vec![];

        for from in reac.from() {
            let child = to_chem_tree(
                reactions,
                reactions.iter().position(|r| r.to().1 == from.1).unwrap(),
                price_list,
                Some(from.0)
            );

            children.push(child);
        }

        ChemNode {
            chem,
            from: None,
            mul: 1.0,
            children,
        }
    }
}

#[derive(Debug, Clone)]
pub struct OrePriceList {
    cost: HashMap<String, f64>,
    per_batch: HashMap<String, usize>,
}

impl OrePriceList {
    /// ```
    /// use aoc_2019::day14::{OrePriceList, Reaction};
    ///
    /// let opl = OrePriceList::new(&vec![
    ///     Reaction::new(&vec![(10, "ORE".to_string())], &(5, "A".to_string())),
    ///     Reaction::new(&vec![(1, "ORE".to_string())], &(1, "B".to_string())),
    ///     Reaction::new(&vec![(10, "ORE".to_string())], &(10, "C".to_string())),
    /// ]);
    ///
    /// assert_eq!(opl.get_cost(&"A".to_string()), Some(&0.5_f64));
    /// assert_eq!(opl.get_cost(&"B".to_string()), Some(&1_f64));
    /// assert_eq!(opl.get_cost(&"C".to_string()), Some(&1_f64));
    ///
    /// assert_eq!(opl.get_required_batch(&"A".to_string()), Some(&5));
    /// assert_eq!(opl.get_required_batch(&"B".to_string()), Some(&1));
    /// assert_eq!(opl.get_required_batch(&"C".to_string()), Some(&10));
    /// ```
    pub fn new(reactions: &Vec<Reaction>) -> OrePriceList {
        let mut cost = HashMap::new();
        let mut per_batch = HashMap::new();
        let ore_str = "ORE".to_string();
        for ore in reactions {
            let from = ore.from();
            let to = ore.to();
            if let Some((divisor, chem_str)) = from.get(0) {
                if *chem_str != ore_str { continue; }

                let divident = to.0;

                let res = divident as f64 / *divisor as f64;

                cost.insert(to.1.clone(), res);
                per_batch.insert(to.1.clone(), to.0);
            }
        }

        OrePriceList {
            cost,
            per_batch
        }
    }

    pub fn get_cost(&self, chem: &String) -> Option<&f64> {
        self.cost.get(chem)
    }

    pub fn get_required_batch(&self, chem: &String) -> Option<&usize> {
        self.per_batch.get(chem)
    }
}

/// ```
/// use aoc_2019::day14::{gather_ore_types, ChemNode};
/// let gathered = gather_ore_types(&ChemNode::new(
///     (1, "FUEL".to_string()),
///     None,
///     1.0,
///     vec![
///         ChemNode::new(
///             (5, "ORE".to_string()),
///             Some("A".to_string()),
///             2.0,
///             vec![],
///         ),
///         ChemNode::new(
///             (7, "ORE".to_string()),
///             Some("B".to_string()),
///             1.0,
///             vec![
///                 ChemNode::new(
///                     (7, "ORE".to_string()),
///                     Some("A".to_string()),
///                     1.0,
///                     vec![],
///                 ),
///             ],
///         ),
///     ],
/// ));
///
/// assert_eq!(gathered.get(&"A".to_string()).unwrap(), &17.0);
/// assert_eq!(gathered.get(&"B".to_string()).unwrap(), &7.0);
/// ```
pub fn gather_ore_types(node: &ChemNode) -> HashMap<String, f64> {
    let mut ore_types = HashMap::new();
    let chem = node.final_chem();

    if chem.1 == &"ORE".to_string() {
        ore_types.insert(node.from().as_ref().unwrap().clone(), node.final_chem().0);
    }

    for child in node.children() {
        for (from, amount) in &gather_ore_types(child) {
            ore_types.insert(from.clone(), match ore_types.get(from) {
                Some(&prev_amount) => prev_amount + *amount,
                None => *amount,
            });
        }
    }

    ore_types
}

/// ```
/// use aoc_2019::day14::{to_chem_tree, Reaction, OrePriceList, calc_ore};
///
/// let reactions = vec![
///     Reaction::new(&vec![(10, "ORE".to_string())], &(10, "A".to_string())),
///     Reaction::new(&vec![(1, "ORE".to_string())], &(1, "B".to_string())),
///     Reaction::new(&vec![(7, "A".to_string()), (1, "B".to_string())], &(1, "C".to_string())),
///     Reaction::new(&vec![(7, "A".to_string()), (1, "C".to_string())], &(1, "D".to_string())),
///     Reaction::new(&vec![(7, "A".to_string()), (1, "D".to_string())], &(1, "E".to_string())),
///     Reaction::new(&vec![(7, "A".to_string()), (1, "E".to_string())], &(1, "FUEL".to_string())),
/// ];
///
/// assert_eq!(calc_ore(&reactions).unwrap(), 31);
/// ```
pub fn calc_ore(reactions: &Vec<Reaction>) -> Option<usize> {
    let price_list = OrePriceList::new(&reactions);
    let fuel_i = reactions.iter().position(|r| r.to().1 == "FUEL".to_string())?;
    let node = to_chem_tree(&reactions, fuel_i, &price_list, None);
    let ore_types = gather_ore_types(&node);

    let mut total_ore_needed = 0;
    for (from, &amount) in &ore_types {
        let batch_f = *price_list.get_required_batch(from).unwrap() as f64;

        if amount % batch_f == 0.0 {
            total_ore_needed += amount as usize;
        } else {
            total_ore_needed += (amount + batch_f - (amount % batch_f)) as usize;
        }
    }

    Some(total_ore_needed)
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Vec<Reaction> {
    input.lines()
        .map(|line| {
            let parts = line.split("=>").collect::<Vec<&str>>();
            let left = manipulate::trim(parts.get(0).expect("Expected left side"), "");
            let right = manipulate::trim(parts.get(1).expect("Expected right side"), "");

            let froms: Vec<Chemical> = left.split(",")
                .map(|fromee_str| {
                    let trimmed = manipulate::trim(fromee_str, "");
                    let parts = trimmed.split(" ").collect::<Vec<&str>>();
                    let number = manipulate::trim(parts.get(0).expect("Expected number"), "").parse::<usize>().expect("Failed parsing number");
                    let chemical = manipulate::trim(parts.get(1).expect("Expected chemical"), "");

                    (number, chemical)
                })
                .collect();

            let right_parts = right.split(" ").collect::<Vec<&str>>();
            let right_number = manipulate::trim(right_parts.get(0).expect("Expected number"), "").parse::<usize>().expect("Failed parsing number");
            let right_chemical = manipulate::trim(right_parts.get(1).expect("Expected chemical"), "");

            Reaction::new(&froms, &(right_number, right_chemical))
        })
        .collect()
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &[Reaction]) -> usize {
    let reactions = input.to_vec().clone();

    calc_ore(&reactions).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_generator() {
        let v = input_generator("2 AB, 3 CD => 5 EF");
        assert_eq!(v[0], Reaction::new(&vec![(2, "AB".to_string()), (3, "CD".to_string())], &(5, "EF".to_string())));
    }
}