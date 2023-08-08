use regex::Regex;
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

pub struct Queue {
    q: Vec<Vec<String>>,
}

impl Queue {
    fn new(q: Vec<Vec<String>>) -> Queue {
        Queue { q }
    }

    fn pop(&mut self) -> (String, i32) {
        let mut cost: i32 = 0;
        while self.q[cost.abs() as usize].is_empty() {
            cost += 1;
        }
        (self.q[cost.abs() as usize].remove(0), -cost)
    }

    fn push(&mut self, regexp: String, cost: i32) {
        self.q[cost.abs() as usize].push(regexp)
    }

    fn is_empty(&self) -> bool {
        self.q.iter().all(|v| v.is_empty())
    }
}

#[inline(never)]
pub fn find_parentheses(regexp: &String, or_only: bool) -> Vec<(usize, usize)> {
    let mut stack: Vec<usize> = Vec::new();
    let mut indices: Vec<(usize, usize)> = Vec::new();

    for (index, c) in regexp.char_indices() {
        if c == '(' {
            stack.push(index);
        } else if c == ')' {
            if let Some(start_index) = stack.pop() {
                indices.push((start_index, index));
            }
        }
    }

    if or_only {
        indices.retain(|(start, end)| {
            regexp[*start..*end + 1].contains("|")
                && !regexp[*start + 1..*end].contains(|c| c == '(' || c == ')')
                && !regexp[*start..*end + 2].ends_with("*")
        })
    }

    indices
}

#[inline(never)]
pub fn is_inside_or(regexp: &String, index: usize) -> bool {
    let positions: Vec<(usize, usize)> = find_parentheses(&regexp, false);
    for (start, end) in positions {
        if start < index && end > index {
            if regexp[start..end + 1].contains("|") {
                return true;
            }
        }
    }

    false
}

#[inline(never)]
pub fn match_all(regexp: &String, positive_set: &Vec<String>) -> bool {
    positive_set
        .iter()
        .all(|x: &String| Regex::new(regexp).unwrap().is_match(x))
}

pub fn match_none(regexp: &String, negative_set: &Vec<String>) -> bool {
    !negative_set
        .iter()
        .any(|x: &String| Regex::new(regexp).unwrap().is_match(x))
}

/*
pub fn is_dead(regexp: &String, positive_set: &Vec<String>, negative_set: &Vec<String>) -> bool {
    let p_regex: String = regexp.replace(r"\x00", r".*");
    let n_regex: String = regexp.replace(r"\x00", r".{0}");
    let pdead: bool = !match_all(&p_regex, &positive_set);
    let ndead: bool = !match_none(&n_regex, &negative_set);

    pdead || ndead
}

pub fn unroll(regexp: &String) -> String {
    // TODO: nested asterisk
    let chars: Vec<char> = regexp.chars().collect();
    let indices: Vec<(usize, usize)> = find_parentheses(&regexp, false);
    let mut replacing: HashSet<(String, String)> = HashSet::new();
    let mut result: String = regexp.clone();

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

pub fn split(regexp: &String) -> Vec<String> {
    let mut results: Vec<String> = Vec::new();
    let positions: Vec<(usize, usize)> = find_parentheses(&regexp, true);

    if positions.is_empty() {
        return vec![regexp.to_string()];
    }

    for (start, end) in positions {
        let or_frag: Vec<String> = regexp[start + 1..end]
            .split("|")
            .map(|x| x.to_string())
            .collect();
        for x in or_frag {
            results.push(format!("{}{}{}", &regexp[..start], x, &regexp[end + 1..]));
        }
    }

    results
}

pub fn is_redundant(regexp: &String, positive_set: &Vec<String>) -> bool {
    let results: Vec<String> = split(&unroll(&regexp));

    for i in &results {
        let p_regex: String = i.replace(r"\x00", r".*");
        if !match_all(&p_regex, &positive_set) {
            return true;
        }
    }

    false
}
*/

#[inline(never)]
pub fn extend(pq: &mut Queue, state: (String, i32), table: &mut HashSet<String>) {
    let occurrences: Vec<(usize, &str)> = state.0.match_indices(r"\x00").collect();
    let all_sub: Vec<(&'static str, i32)> = vec![
        (r"0", -1),
        (r"1", -1),
        (r"(\x00)*", -1),
        (r"\x00\x00", -1),
        (r"(\x00|\x00)", -1),
    ];

    for (index, _block) in occurrences {
        for (s, cost) in &all_sub {
            if is_inside_or(&state.0, index) && s == &r"(\x00|\x00)" {
                let ext_regexp: &String =
                    &format!(r"{}\x00|\x00{}", &state.0[..index], &state.0[index + 4..]);
                let extended_state: (&String, i32) = (&ext_regexp, state.1 + cost);
                if !table.contains(ext_regexp) {
                    table.insert(ext_regexp.to_string());
                    pq.push(extended_state.0.to_string(), extended_state.1);
                }
            } else {
                let ext_regexp: &String =
                    &format!("{}{}{}", &state.0[..index], s, &state.0[index + 4..]); // \x00算四個字元
                let extended_state: (&String, i32) = (&ext_regexp, state.1 + cost);
                if !table.contains(ext_regexp) {
                    table.insert(ext_regexp.to_string());
                    pq.push(extended_state.0.to_string(), extended_state.1);
                }
            }
        }
    }
}

#[inline(never)]
pub fn synth(positive_set: &Vec<String>, negative_set: &Vec<String>, debug: bool) -> (String, i32) {
    let (init_cost, init_regexp) = (0, String::from(r"^\x00$"));
    let mut pq: Queue = Queue::new(vec![vec![]; 100]);
    let (mut total, mut dead, mut redundant) = (0, 0, 0);
    let mut table: HashSet<String> = HashSet::new();

    pq.push(init_regexp, init_cost);
    while !pq.is_empty() {
        let curr_state: (String, i32) = pq.pop();
        let curr_regexp: &String = &curr_state.0;
        if debug {
            println!("{}", curr_regexp);
        }

        if !curr_regexp.contains(r"\x00") {
            if match_all(&curr_regexp, &positive_set) && match_none(&curr_regexp, &negative_set) {
                println!("Total: {}, Dead: {}, Redundant: {}", total, dead, redundant);
                return curr_state;
            }
        // } else if is_dead(&curr_regexp, &positive_set, &negative_set) {
        //     dead += 1;
        // } else if is_redundant(&curr_regexp, &positive_set) {
        //     redundant += 1;
        } else {
            extend(&mut pq, curr_state, &mut table);
        }
        total += 1;
    }
    ("NO SOLUTION".to_string(), 0)
}
