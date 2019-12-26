use failure::Error;

#[derive(Debug, Fail, PartialEq)]
pub enum OrbitErr {
    #[fail(display = "Parse error")]
    ParseError,
    #[fail(display = "An orbited entity can only exist once")]
    OrbitedDupe,
    #[fail(display = "Node index out of bounds")]
    NodeOutOfBounds,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Node<'a > {
    pub name: &'a str,
    pub parent: Option<usize>,
}

impl Node<'_> {
    pub fn new(name: &str, parent: Option<usize>) -> Node {
        Node { name, parent }
    }
}

/// ```
/// use advent::day_06::{orbits_to_nodes, Orbit, Node};
///
/// let orbits1 = vec![
///     Orbit::new("COM)B").unwrap(),
///     Orbit::new("B)C").unwrap(),
///     Orbit::new("C)D").unwrap(),
///     Orbit::new("D)E").unwrap(),
///     Orbit::new("E)F").unwrap(),
///     Orbit::new("B)G").unwrap(),
///     Orbit::new("D)I").unwrap(),
///     Orbit::new("E)J").unwrap(),
///     Orbit::new("J)K").unwrap(),
///     Orbit::new("K)L").unwrap(),
/// ];
/// let nodes1 = orbits_to_nodes(&orbits1).unwrap();
///
/// assert_eq!(nodes1[..], vec![
///     Node { name: "COM", parent: None },  // 0
///     Node { name: "B", parent: Some(0) }, // 1
///     Node { name: "C", parent: Some(1) }, // 2
///     Node { name: "D", parent: Some(2) }, // 3
///     Node { name: "E", parent: Some(3) }, // 4
///     Node { name: "F", parent: Some(4) }, // 5
///     Node { name: "G", parent: Some(1) }, // 6
///     Node { name: "I", parent: Some(3) }, // 7
///     Node { name: "J", parent: Some(4) }, // 8
///     Node { name: "K", parent: Some(8) }, // 9
///     Node { name: "L", parent: Some(9) }, // 10
/// ][..]);
///
/// let orbits2 = vec![
///     Orbit::new("COM)PWZ").unwrap(),
///     Orbit::new("PWZ)CPF").unwrap(),
///     Orbit::new("CPF)DH3").unwrap(),
///     Orbit::new("DH3)XC8").unwrap(),
///     Orbit::new("XC8)J5P").unwrap(),
///     Orbit::new("J5P)N86").unwrap(),
///     Orbit::new("N86)NL2").unwrap(),
///     Orbit::new("NL2)D6G").unwrap(),
///     Orbit::new("D6G)P6C").unwrap(),
///     Orbit::new("P6C)BJF").unwrap(),
///     Orbit::new("BJF)TKC").unwrap(),
///     Orbit::new("TKC)CKB").unwrap(),
///     Orbit::new("CKB)8CC").unwrap(),
///     Orbit::new("8CC)4DY").unwrap(),
///     Orbit::new("4DY)G36").unwrap(),
///     Orbit::new("G36)Q3T").unwrap(),
///     Orbit::new("Q3T)RDG").unwrap(),
///     Orbit::new("RDG)L6M").unwrap(),
///     Orbit::new("L6M)KVN").unwrap(),
/// ];
///
/// let nodes2 = orbits_to_nodes(&orbits2).unwrap();
///
/// assert_eq!(nodes2[..], vec![
///     Node { name: "COM", parent: None },      // 0
///     Node { name: "PWZ", parent: Some(0) },   // 1
///     Node { name: "CPF", parent: Some(1) },   // 2
///     Node { name: "DH3", parent: Some(2) },   // 3
///     Node { name: "XC8", parent: Some(3) },   // 4
///     Node { name: "J5P", parent: Some(4) },   // 5
///     Node { name: "N86", parent: Some(5) },   // 6
///     Node { name: "NL2", parent: Some(6) },   // 7
///     Node { name: "D6G", parent: Some(7) },   // 8
///     Node { name: "P6C", parent: Some(8) },   // 9
///     Node { name: "BJF", parent: Some(9) },   // 10
///     Node { name: "TKC", parent: Some(10) },  // 11
///     Node { name: "CKB", parent: Some(11) },  // 12
///     Node { name: "8CC", parent: Some(12) },  // 13
///     Node { name: "4DY", parent: Some(13) },  // 14
///     Node { name: "G36", parent: Some(14) },  // 15
///     Node { name: "Q3T", parent: Some(15) },  // 16
///     Node { name: "RDG", parent: Some(16) },  // 17
///     Node { name: "L6M", parent: Some(17) },  // 18
///     Node { name: "KVN", parent: Some(18) },  // 19
/// ][..]);
/// ```
pub fn orbits_to_nodes<'a>(orbits: &'a Vec<Orbit>) -> Result<Vec<Node<'a>>, OrbitErr> {
    let mut nodes: Vec<Node> = vec![];

    for orbit in orbits {
        let parent: Option<usize>;
        let orbited_exists = nodes.iter().position(|n| n.name.to_string() == orbit.orbited);
        if let None = orbited_exists {
            // No orbited node exists, create it
            let orbited_node = Node::new(
                &orbit.orbited[..],
                None
            );

            nodes.push(orbited_node);
            parent = Some(nodes.len() - 1);
        } else {
            // Orbited exists, use it
            parent = orbited_exists;
        }

        let orbiter_exists = nodes.iter().position(|n| n.name.to_string() == orbit.orbiter);
        match orbiter_exists {
            None => {
                // No orbiter node exists, create it
                let orbiter_node = Node::new(
                    &orbit.orbiter[..],
                    parent,
                );

                nodes.push(orbiter_node);
            },
            Some(i) => {
                // Orbiter node exists, if it lacks a parent update it
                let node = nodes.get_mut(i).ok_or(OrbitErr::NodeOutOfBounds)?;

                if let None = node.parent {
                    node.parent = parent;
                }
            }
        }
    }

    Ok(nodes)
}

/// ```
/// use advent::day_06::{get_orbit_count, Node};
///
/// //         G - H       J - K - L
/// //        /           /
/// // COM - B - C - D - E - F
/// //               \
/// //                 I
///
/// let nodes1 = vec![
///     Node { name: "COM", parent: None },  // 0
///     Node { name: "B", parent: Some(0) }, // 1
///     Node { name: "C", parent: Some(1) }, // 2
///     Node { name: "D", parent: Some(2) }, // 3
///     Node { name: "E", parent: Some(3) }, // 4
///     Node { name: "F", parent: Some(4) }, // 5
///     Node { name: "G", parent: Some(1) }, // 6
///     Node { name: "H", parent: Some(6) }, // 7
///     Node { name: "I", parent: Some(3) }, // 8
///     Node { name: "J", parent: Some(4) }, // 9
///     Node { name: "K", parent: Some(9) }, // 10
///     Node { name: "L", parent: Some(10) }, // 11
/// ];
///
/// assert_eq!(get_orbit_count(&nodes1, 0), Ok(0));
/// assert_eq!(get_orbit_count(&nodes1, 1), Ok(1));
/// assert_eq!(get_orbit_count(&nodes1, 2), Ok(2));
/// assert_eq!(get_orbit_count(&nodes1, 3), Ok(3));
/// assert_eq!(get_orbit_count(&nodes1, 4), Ok(4));
/// assert_eq!(get_orbit_count(&nodes1, 5), Ok(5));
/// assert_eq!(get_orbit_count(&nodes1, 6), Ok(2));
/// assert_eq!(get_orbit_count(&nodes1, 7), Ok(3));
/// assert_eq!(get_orbit_count(&nodes1, 8), Ok(4));
/// assert_eq!(get_orbit_count(&nodes1, 9), Ok(5));
/// assert_eq!(get_orbit_count(&nodes1, 10), Ok(6));
/// assert_eq!(get_orbit_count(&nodes1, 11), Ok(7));
/// ```
pub fn get_orbit_count(orbits: &Vec<Node>, index: usize) -> Result<u64, OrbitErr> {
    let subject = orbits.get(index).ok_or(OrbitErr::NodeOutOfBounds)?;

    let mut orbit_cnt = 0;

    if let Some(i) = subject.parent {
        orbit_cnt = 1;
        orbit_cnt += get_orbit_count(orbits, i)?;
    }

    Ok(orbit_cnt)
}

#[derive(Debug, PartialEq, Clone)]
pub struct Orbit {
    pub orbited: String,
    pub orbiter: String,
}

impl Orbit {

    /// ```
    /// use advent::day_06::{Orbit, OrbitErr};
    ///
    /// assert_eq!(Orbit::new("XLG)95G").unwrap(), Orbit { orbited: "XLG".to_string(), orbiter: "95G".to_string() });
    /// assert_eq!(Orbit::new("XLG_95G"), Err(OrbitErr::ParseError));
    /// ```
    pub fn new(orbit_txt: &str) -> Result<Orbit, OrbitErr> {
        let pieces: Vec<String> = orbit_txt
            .to_string()
            .split(")")
            .map(|s| s.to_string())
            .collect();

        if pieces.len() != 2 {
            return Err(OrbitErr::ParseError);
        } else {
            Ok(Orbit {
                orbited: pieces.get(0).ok_or(OrbitErr::ParseError)?.to_owned(),
                orbiter: pieces.get(1).ok_or(OrbitErr::ParseError)?.to_owned(),
            })
        }
    }
}

/// ```
/// use advent::day_06::{get_orbits_from_input, Orbit};
///
/// let s = "XLG)95G\nW3V)ZZ3\nZM3)Q4Q".to_string();
/// let orbits = get_orbits_from_input(&s);
/// assert_eq!(orbits.len(), 3);
/// assert_eq!(*orbits.get(0).unwrap(), Orbit { orbited: "XLG".to_string(), orbiter: "95G".to_string() });
/// assert_eq!(*orbits.get(1).unwrap(), Orbit { orbited: "W3V".to_string(), orbiter: "ZZ3".to_string() });
/// assert_eq!(*orbits.get(2).unwrap(), Orbit { orbited: "ZM3".to_string(), orbiter: "Q4Q".to_string() });
/// ```
pub fn get_orbits_from_input(input: &String) -> Vec<Orbit> {
    input
        .lines()
        .map(|line| Orbit::new(&line[..]))
        .filter_map(Result::ok)
        .collect()
}

/// ```
/// use advent::day_06::{Node, calc_total_orbits};
///
/// let nodes = vec![
///     Node { name: "COM", parent: None },  // 0
///     Node { name: "B", parent: Some(0) }, // 1
///     Node { name: "C", parent: Some(1) }, // 2
///     Node { name: "D", parent: Some(2) }, // 3
///     Node { name: "E", parent: Some(3) }, // 4
///     Node { name: "F", parent: Some(4) }, // 5
///     Node { name: "G", parent: Some(1) }, // 6
///     Node { name: "H", parent: Some(6) }, // 7
///     Node { name: "I", parent: Some(3) }, // 8
///     Node { name: "J", parent: Some(4) }, // 9
///     Node { name: "K", parent: Some(9) }, // 10
///     Node { name: "L", parent: Some(10) }, // 11
/// ];
///
/// assert_eq!(calc_total_orbits(&nodes).unwrap(), 42);
/// ```
pub fn calc_total_orbits(nodes: &Vec<Node>) -> Result<u64, Error> {
    let mut orbit_count: u64 = 0;

    for i in 0..nodes.len() {
        orbit_count += get_orbit_count(&nodes, i)?;
    }

    Ok(orbit_count)
}

/// ```
/// use advent::day_06::{get_orbits_from_input, orbits_to_nodes, find_root, traverse_children_until, path_diff, PathDiffFavor, join_diff_paths, trim_ends};
///
/// //                          YOU
/// //                         /
/// //        G - H       J - K - L
/// //       /           /
/// //COM - B - C - D - E - F
/// //               \
/// //                I - SAN
///
/// let orbit_string = "COM)B
/// B)C
/// C)D
/// D)E
/// E)F
/// B)G
/// G)H
/// D)I
/// E)J
/// J)K
/// K)L
/// K)YOU
/// I)SAN".to_string();
///
/// let orbits = get_orbits_from_input(&orbit_string);
///
/// let nodes = orbits_to_nodes(&orbits).unwrap();
/// let from = nodes.iter().position(|n| n.name == "YOU").unwrap();
/// let end = nodes.iter().position(|n| n.name == "SAN").unwrap();
///
/// let root = find_root(&nodes).unwrap();
/// let root_to_end = traverse_children_until(&nodes, root, end).unwrap().unwrap();
/// let root_to_start = traverse_children_until(&nodes, root, from).unwrap().unwrap();
///
/// let diff_path1 = path_diff(&root_to_start, &root_to_end, PathDiffFavor::First).expect("first failed");
/// let diff_path2 = path_diff(&root_to_start, &root_to_end, PathDiffFavor::Second).expect("second failed");
///
/// let joined = join_diff_paths(&diff_path1, &diff_path2).unwrap();
/// let trimmed = trim_ends(&joined);
///
/// assert_eq!(trimmed[..], vec![
///     "K",
///     "J",
///     "E",
///     "D",
///     "I",
/// ][..]);
/// ```
pub fn traverse_children_until<'a>(nodes: &'a Vec<Node>, current: usize, end: usize) -> Result<Option<Vec<&'a str>>, OrbitErr> {
    let current_node = nodes.get(current).ok_or(OrbitErr::NodeOutOfBounds)?;
    let mut path: Vec<&'a str> = vec![current_node.name.clone()];

    // Are we there yet?
    if current == end {
        return Ok(Some(path));
    }

    // Iterate over the children of this node
    let children = get_children(nodes, current);

    for child in children {
        // Find the children of this child
        if let Some(child_path) = traverse_children_until(nodes, child, end)? {
            // Target found, return
            for child_path_item in child_path {
                path.push(child_path_item);
            }

            return Ok(Some(path))
        }
    }

    Ok(None)
}

/// ```
/// use advent::day_06::trim_ends;
/// let trimmed = trim_ends(&vec!["A", "B", "C", "D"]);
/// assert_eq!(trimmed[..], vec!["B", "C"][..]);
/// ```
pub fn trim_ends<'a>(subject: &Vec<&'a str>) -> Vec<&'a str> {
    subject.iter().skip(1).take(subject.len() - 2).map(|s| *s).collect()
}

/// ```
/// use advent::day_06::join_diff_paths;
/// let diff1 = vec!["COM", "B", "C", "D", "E", "J", "K", "YOU"];
/// let diff2 = vec!["COM", "B", "C", "D", "I", "SAN"];
///
/// let result = join_diff_paths(&diff1, &diff2).unwrap();
///
/// assert_eq!(result[..], vec![
///     "YOU",
///     "K",
///     "J",
///     "E",
///     "D",
///     "I",
///     "SAN",
/// ][..]);
/// ```
pub fn join_diff_paths<'a>(diff1: &Vec<&'a str>, diff2: &Vec<&'a str>) -> Result<Vec<&'a str>, OrbitErr> {
    let mut joined: Vec<&'a str> = vec![];

    let start_of_diff = diff1
        .iter()
        .enumerate()
        .position(|(i, s)| s != diff2.get(i).unwrap_or(&"INVALID"));

    if let Some(d) = start_of_diff {
        let common = diff1.get(d - 1).ok_or(OrbitErr::NodeOutOfBounds)?;

        let reverse_diff1: Vec<&'a str> = diff1.iter().rev().map(|s| *s).collect();
        for i in 0..(reverse_diff1.len() - d) {
            joined.push(reverse_diff1.get(i).ok_or(OrbitErr::NodeOutOfBounds)?);
        }

        joined.push(common);

        for i in d..diff2.len() {
            joined.push(diff2.get(i).ok_or(OrbitErr::NodeOutOfBounds)?);
        }

        return Ok(joined);
    }

    Err(OrbitErr::ParseError)
}

pub enum PathDiffFavor {
    First,
    Second,
}

/// ```
/// use advent::day_06::{path_diff, PathDiffFavor};
/// let path1 = vec![
///     "A",
///     "B",
///     "C",
///     "D",
/// ];
///
/// let path2 = vec![
///     "A",
///     "B",
///     "E",
///     "F",
/// ];
///
/// let res = path_diff(&path1, &path2, PathDiffFavor::First).unwrap();
///
/// assert_eq!(res[..], vec![
///     "B",
///     "C",
///     "D",
/// ][..]);
/// ```
pub fn path_diff<'a>(path1: &Vec<&'a str>, path2: &Vec<&'a str>, path_diff_favor: PathDiffFavor) -> Option<Vec<&'a str>> {
    let mut last_same: Option<&'a str> = None;
    let mut diff: Vec<&'a str> = vec![];
    let len = match path1.len() > path2.len() {
        true => path1.len(),
        false => path2.len(),
    };

    for i in 0..len {
        let item1 = path1.get(i);
        let item2 = path2.get(i);

        match (item1, item2) {
            (Some(i1), Some(i2)) => {
                if i1 == i2 {
                    last_same = Some(*i1);
                } else {
                    if let PathDiffFavor::First = path_diff_favor {
                        diff.push(*i1);
                    } else {
                        diff.push(*i2);
                    }
                }
            },
            (Some(i1), None) => {
                if let PathDiffFavor::First = path_diff_favor {
                    diff.push(*i1);
                }
            },
            (None, Some(i2)) => {
                if let PathDiffFavor::Second = path_diff_favor {
                    diff.push(*i2);
                }
            },
            _ => panic!("Invalid state")
        }
    }

    // Prepend last_same
    let mut diff_with_last_same: Vec<&'a str> = vec![last_same.expect("No similar nodes")];
    for item in diff {
        diff_with_last_same.push(item);
    }

    Some(diff_with_last_same)
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Orbit> {
    input
        .lines()
        .map(|line| Orbit::new(&line[..]))
        .filter_map(Result::ok)
        .collect()
}

pub fn get_children(nodes: &Vec<Node>, parent: usize) -> Vec<usize> {
    let mut children: Vec<usize> = vec![];

    for (i, node) in nodes.iter().enumerate() {
        if let Some(p) = node.parent {
            if p == parent {
                children.push(i);
            }
        }
    }

    children
}

pub fn find_root(nodes: &Vec<Node>) -> Option<usize> {
    nodes.iter().position(|n| n.parent.is_none())
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &[Orbit]) -> u64 {
    let orbits = input.to_vec();
    let nodes = orbits_to_nodes(&orbits).unwrap();

    calc_total_orbits(&nodes).unwrap()
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &[Orbit]) -> usize {
    let orbits = input.to_vec();
    let nodes = orbits_to_nodes(&orbits).unwrap();

    let from = nodes.iter().position(|n| n.name == "YOU").unwrap();
    let end = nodes.iter().position(|n| n.name == "SAN").unwrap();
    let root = find_root(&nodes).unwrap();

    let root_to_end = traverse_children_until(&nodes, root, end).unwrap().unwrap();
    let root_to_start = traverse_children_until(&nodes, root, from).unwrap().unwrap();

    let diff_path1 = path_diff(&root_to_start, &root_to_end, PathDiffFavor::First).expect("first failed");
    let diff_path2 = path_diff(&root_to_start, &root_to_end, PathDiffFavor::Second).expect("second failed");
    let joined = join_diff_paths(&diff_path1, &diff_path2).unwrap();
    let trimmed = trim_ends(&joined);

    trimmed.len() - 1
}
