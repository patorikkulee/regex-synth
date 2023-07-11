use priority_queue::PriorityQueue;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

// pub struct State {
//     cost: i32,
//     regexp: String,
//     is_leaf: bool,
// }

// impl State {
//     fn new(cost: i32, regexp: String) -> State {
//         let is_leaf: bool = !regexp.contains('\x00');

//         State {
//             cost,
//             regexp,
//             is_leaf,
//         }
//     }
// }

pub fn all_sub() -> HashMap<String, i32> {
    let mut all_sub: HashMap<String, i32> = HashMap::new();
    const COST: i32 = -1;
    all_sub.insert(r"0".to_owned(), COST);
    all_sub.insert(r"1".to_owned(), COST);
    all_sub.insert(r"(\x00)*".to_owned(), COST);
    all_sub.insert(r"\x00\x00".to_owned(), COST);
    all_sub.insert(r"(\x00)|(\x00)".to_owned(), COST);

    all_sub
}

pub fn find_parentheses(regexp: String) -> Vec<(usize, usize)> {
    let mut stack: Vec<usize> = Vec::new();
    let mut indices: Vec<(usize, usize)> = Vec::new();
    let chars: Vec<char> = regexp.chars().collect();

    for (index, &c) in chars.iter().enumerate() {
        if c == '(' {
            stack.push(index);
        } else if c == ')' {
            if let Some(start_index) = stack.pop() {
                indices.push((start_index, index));
            }
        }
    }

    indices
}

// pub fn IsInsideOr() {}

pub fn match_all(regexp: String, positive_set: Vec<String>) -> bool {
    positive_set
        .into_iter()
        .all(|x: String| Regex::new(&regexp).unwrap().is_match(&x))
}

pub fn match_none(regexp: String, negative_set: Vec<String>) -> bool {
    !negative_set
        .into_iter()
        .any(|x: String| Regex::new(&regexp).unwrap().is_match(&x))
}

pub fn is_dead(regexp: String, positive_set: Vec<String>, negative_set: Vec<String>) -> bool {
    let p_regex: String = regexp.replace(r"\x00", r".*");
    let n_regex: String = regexp.replace(r"\x00", r".{0}"); // change to other representation
    let pdead: bool = !match_all(p_regex, positive_set);
    let ndead: bool = !match_none(n_regex, negative_set);

    pdead || ndead
}

pub fn unroll(regexp: String) -> String {
    let chars: Vec<char> = regexp.chars().collect();
    let indices: Vec<(usize, usize)> = find_parentheses(regexp.clone());
    let mut replacing: HashSet<(String, String)> = HashSet::new();
    let mut result: String = String::from(regexp.clone());

    for &(start, end) in indices.iter() {
        if chars.get(end + 1) == Some(&'*') {
            let old_str: &str = &regexp[start..end + 1];
            let new_str: String = format!("{}{}{}", old_str, old_str, old_str);
            replacing.insert((old_str.to_string(), new_str));
        }
    }

    for (old_str, new_str) in &replacing {
        result = result.replace(old_str, new_str);
    }

    result
}

pub fn find_split_sets(regexp: String) -> HashMap<usize, HashMap<String, (usize, usize)>> {
    let positions: Vec<(usize, usize)> = find_parentheses(regexp.clone());
    let chars: Vec<char> = regexp.chars().collect();
    let mut results: HashMap<usize, HashMap<String, (usize, usize)>> = HashMap::new();

    for (index, &c) in chars.iter().enumerate() {
        if c == '|' {
            let left: &(usize, usize) = positions
                .iter()
                .filter(|p: &&(usize, usize)| p.1 == index - 1)
                .next()
                .unwrap();

            let right: &(usize, usize) = positions
                .iter()
                .filter(|p: &&(usize, usize)| p.0 == index + 1)
                .next()
                // .map(|p| p.1)
                .unwrap();

            let span: (usize, usize) = (left.0, right.1);

            let mut or_info: HashMap<String, (usize, usize)> = HashMap::new();
            or_info.insert("left".to_owned(), *left);
            or_info.insert("right".to_owned(), *right);
            or_info.insert("span".to_owned(), span);

            results.insert(index, or_info);
        }
    }

    results
}

pub fn split(regexp: String) -> Vec<String> {
    let mut results: Vec<String> = Vec::new();
    let positions: HashMap<usize, HashMap<String, (usize, usize)>> =
        find_split_sets(regexp.clone());

    if positions.len() == 0 {
        return vec![regexp];
    }

    for (_index, ranges) in &positions {
        let lspan: (usize, usize) = match ranges.get("left") {
            Some((start, end)) => (*start, *end),
            None => (0, 0),
        };
        let rspan: (usize, usize) = match ranges.get("right") {
            Some((start, end)) => (*start, *end),
            None => (0, 0),
        };

        let left_str: &String = &regexp[lspan.0..lspan.1 + 1].to_owned();
        let right_str: &String = &regexp[rspan.0..rspan.1 + 1].to_owned();
        // println!("{}, {}", left_str, right_str);

        results.push(format!(
            "{}{}{}",
            &regexp[..lspan.0],
            left_str,
            &regexp[rspan.1 + 1..]
        ));
        results.push(format!(
            "{}{}{}",
            &regexp[..lspan.0],
            right_str,
            &regexp[rspan.1 + 1..]
        ));
    }

    results
}

pub fn is_redundant(regexp: String, positive_set: Vec<String>) -> bool {
    let results: Vec<String> = split(unroll(regexp));

    for i in &results {
        let p_regex: String = i.replace(r"\x00", r".*");
        if !match_all(p_regex, positive_set.clone()) {
            return true;
        }
    }

    false
}

pub fn extend(
    pq: &mut PriorityQueue<String, i32>,
    state: (String, i32),
    table: &mut HashSet<String>,
) {
    let occurrences: Vec<(usize, &str)> = state.0.match_indices(r"\x00").collect();
    let all_sub: HashMap<String, i32> = all_sub();

    for (index, _block) in occurrences {
        for (s, cost) in &all_sub {
            let ext_regexp: String = format!("{}{}{}", &state.0[..index], s, &state.0[index + 4..]); // \x00算四個字元哭啊
            let extended_state: (String, i32) = (ext_regexp.clone(), state.1 + cost);
            if !table.contains(&ext_regexp) {
                table.insert(ext_regexp.clone());
                pq.push(extended_state.0, extended_state.1);
            }
        }
    }
}

pub fn synth(positive_set: Vec<String>, negative_set: Vec<String>, debug: bool) -> (String, i32) {
    let (init_cost, init_regexp) = (0, String::from(r"^\x00$"));
    let mut pq: PriorityQueue<String, i32> = PriorityQueue::new();
    let (mut total, mut dead, mut redundant) = (0, 0, 0);
    let mut table: HashSet<String> = HashSet::new();

    pq.push(init_regexp, init_cost);
    while !pq.is_empty() {
        let curr_state: Option<(String, i32)> = pq.pop();
        let curr_regexp: String = curr_state.clone().unwrap().0.to_string();
        if debug {
            println!("{}", curr_regexp);
        }

        if !curr_regexp.contains(r"\x00") {
            if match_all(curr_regexp.clone(), positive_set.clone())
                && match_none(curr_regexp.clone(), negative_set.clone())
            {
                println!("Total: {}, Dead: {}, Redundant: {}", total, dead, redundant);
                return curr_state.clone().unwrap();
            }
        } else {
            if is_dead(
                curr_regexp.clone(),
                positive_set.clone(),
                negative_set.clone(),
            ) {
                dead += 1;
            } else if is_redundant(curr_regexp.clone(), positive_set.clone()) {
                redundant += 1;
            } else {
                extend(&mut pq, curr_state.unwrap(), &mut table);
            }
        }
        total += 1;
    }

    ("NO SOLUTION".to_string(), 0)
}
