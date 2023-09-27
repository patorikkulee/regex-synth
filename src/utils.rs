#[allow(dead_code)]
use flamer::flame;
use rand::{seq::index, Rng};
use random_string::generate;
use regex::Regex;
use std::{collections::HashSet, collections::LinkedList, vec};

// substitute, cost, pan_dist, pan_backwards
const ALL_SUB: [(&'static str, usize, usize, bool); 5] = [
    (r"0", 1, 3, true),
    (r"1", 1, 3, true),
    (r"(\x00)*", 1, 3, false),
    (r"\x00\x00", 1, 4, false),
    (r"(\x00|\x00)", 1, 7, false),
];

#[derive(Clone, PartialEq, Debug)]
pub struct State {
    pub cost: usize,
    pub regexp: String,
    is_leaf: bool,
    pub parentheses: Vec<(usize, usize)>,
}

impl State {
    pub fn new(cost: usize, regexp: String, parentheses: Vec<(usize, usize)>) -> State {
        let is_leaf: bool = !regexp.contains(r"\x00");

        State {
            cost,
            regexp,
            is_leaf,
            parentheses,
        }
    }
}

pub struct Queue {
    q: Vec<Vec<State>>,
    cost: usize,
    index: usize,
}

impl Queue {
    pub fn new(q: Vec<Vec<State>>, cost: usize, index: usize) -> Queue {
        Queue { q, cost, index }
    }

    pub fn pop(&mut self) -> State {
        if let Some(level) = self.q.get_mut(self.cost) {
            if let Some(item) = level.get(self.index) {
                self.index += 1;
                return item.clone();
            }
        }

        self.cost += 1;
        self.index = 0;

        if let Some(level) = self.q.get(self.cost) {
            if let Some(item) = level.get(self.index) {
                self.index += 1;
                return item.clone();
            }
        }

        State::new(0, "".to_string(), Vec::new())
        // let mut cost: usize = 0;
        // // while self.q[cost.abs() as usize].is_empty() {
        // while self.q[cost].is_empty() {
        //     cost += 1;
        // }
        // // (self.q[cost.abs() as usize].remove(0), -cost)
        // (self.q[cost].remove(0), cost)
    }

    pub fn push(&mut self, s: State) {
        // self.q[cost.abs() as usize].push(regexp)
        self.q[s.cost].push(s)
    }

    pub fn is_empty(&self) -> bool {
        self.q.iter().all(|v| v.is_empty())
    }
}

#[inline(never)]
#[flame]
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
#[flame]
// check if the extension will be implemented inside or, if yes then the modification will be different e.g., (1|2)* -> (1|2|3)*
pub fn is_inside_or(s: &State, index: usize) -> bool {
    for (start, end) in &s.parentheses {
        if start < &index && end > &index {
            if s.regexp.chars().nth(index - start) == Some('|') {
                return true;
            }
        }
    }

    false
}

#[inline(never)]
#[flame]
pub fn update_parentheses(
    parentheses: &mut Vec<(usize, usize)>,
    x: usize,
    pan_dist: usize,
    pan_backwards: bool,
) -> Vec<(usize, usize)> {
    for tuple in parentheses.iter_mut() {
        if tuple.0 > x && pan_backwards {
            tuple.0 = tuple.0 - pan_dist;
        }
        if tuple.1 > x && pan_backwards {
            tuple.1 = tuple.1 - pan_dist;
        }
        if tuple.0 > x && !pan_backwards {
            tuple.0 = tuple.0 + pan_dist;
        }
        if tuple.1 > x && !pan_backwards {
            tuple.1 = tuple.1 + pan_dist;
        }
    }
    parentheses.clone()
}

#[inline(never)]
#[flame]
pub fn match_all(regexp: &str, positive_set: &Vec<String>) -> bool {
    positive_set
        .iter()
        .all(|x: &String| Regex::new(regexp).unwrap().is_match(x))
}

#[flame]
pub fn match_none(regexp: &str, negative_set: &Vec<String>) -> bool {
    !negative_set
        .iter()
        .any(|x: &String| Regex::new(regexp).unwrap().is_match(x))
}

#[flame]
pub fn is_dead(regexp: &String, positive_set: &Vec<String>, negative_set: &Vec<String>) -> bool {
    // let p_regex: &str = &regexp.replace(r"\x00", r".*");
    let n_regex: &str = &regexp.replace(r"\x00", r".{0}");
    // let pdead: bool = !match_all(p_regex, &positive_set);
    let ndead: bool = !match_none(n_regex, &negative_set);

    ndead
}

/*
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
#[flame]
pub fn extend(pq: &mut Queue, state: State, table: &mut HashSet<String>) {
    let occurrences: &Vec<(usize, &str)> = &state.regexp.match_indices(r"\x00").collect();

    for (index, _block) in occurrences {
        for (s, cost, pan_dist, pan_backwards) in &ALL_SUB {
            if is_inside_or(&state, *index) && s == &r"(\x00|\x00)" {
                let ext_regexp: &String = &format!(
                    r"{}\x00|\x00{}",
                    &state.regexp[..*index],
                    &state.regexp[*index + 4..]
                );
                let mut ext_parentheses = state.parentheses.clone();
                update_parentheses(&mut ext_parentheses, *index, 5, *pan_backwards);

                if !table.contains(ext_regexp) {
                    let extended_state: State =
                        State::new(state.cost + cost, ext_regexp.to_string(), ext_parentheses);
                    table.insert(ext_regexp.to_string());
                    pq.push(extended_state);
                }
            } else {
                let ext_regexp: String = format!(
                    "{}{}{}",
                    &state.regexp[..*index],
                    s,
                    &state.regexp[index + 4..]
                ); // \x00算四個字元
                let mut ext_parentheses = state.parentheses.clone();
                update_parentheses(&mut ext_parentheses, *index, *pan_dist, *pan_backwards);
                if s == &r"(\x00)*" {
                    ext_parentheses.push((*index, *index + 5))
                } else if s == &r"(\x00|\x00)" {
                    ext_parentheses.push((*index, *index + 10))
                }

                let extended_state: State =
                    State::new(state.cost + cost, ext_regexp.to_string(), ext_parentheses);
                if !table.contains(&ext_regexp) {
                    table.insert(ext_regexp.to_string());
                    pq.push(extended_state);
                }
            }
        }
    }
}

#[inline(never)]
#[flame]
pub fn synth(positive_set: &Vec<String>, negative_set: &Vec<String>, debug: bool) -> State {
    // let (init_cost, init_regexp) = (0, String::from(r"^\x00$"));
    let init_state: State = State::new(0, r"^\x00$".to_string(), Vec::new());
    let mut pq: Queue = Queue::new(vec![vec![]; 10], 0, 0);
    let (mut total, mut dead, mut redundant) = (0, 0, 0);
    let mut table: HashSet<String> = HashSet::new();

    pq.push(init_state);
    while !pq.is_empty() {
        let curr_state: State = pq.pop();
        // let curr_regexp: &String = &curr_state.0;
        if debug {
            println!(
                "{}, {}, {:?}",
                curr_state.regexp, curr_state.cost, curr_state.parentheses
            );
        }

        // if !curr_state.regexp.contains(r"\x00") {
        if curr_state.is_leaf {
            if match_all(&curr_state.regexp, &positive_set)
                && match_none(&curr_state.regexp, &negative_set)
            {
                println!("Total: {}, Dead: {}, Redundant: {}", total, dead, redundant);
                return curr_state;
            }
        // } else if is_dead(&curr_state.regexp, &positive_set, &negative_set) {
        //     dead += 1;
        // } else if is_redundant(&curr_regexp, &positive_set) {
        //     redundant += 1;
        } else {
            extend(&mut pq, curr_state, &mut table);
        }
        total += 1;
        // break;
    }
    State::new(0, "".to_string(), Vec::new())
}

pub fn negative_examples(condition: &str, set_len: usize) -> Vec<String> {
    let charset: &str = "01";
    let mut examples: Vec<String> = Vec::new();
    let mut curr_example: String = "".to_string();

    if condition == "start_with_0" {
        while examples.len() < set_len {
            curr_example = generate(rand::thread_rng().gen_range(1..50), charset);
            if !curr_example.starts_with("0") && !examples.contains(&curr_example) {
                examples.push(curr_example);
            }
        }
    } else if condition == "end_with_01" {
        while examples.len() < set_len {
            curr_example = generate(rand::thread_rng().gen_range(1..50), charset);
            if !curr_example.ends_with("01") && !examples.contains(&curr_example) {
                examples.push(curr_example);
            }
        }
    } else if condition == "containing_0101" {
        while examples.len() < set_len {
            curr_example = generate(rand::thread_rng().gen_range(1..50), charset);
            if !curr_example.contains("0101") && !examples.contains(&curr_example) {
                examples.push(curr_example);
            }
        }
    } else if condition == "begin_1_end_0" {
        while examples.len() < set_len {
            curr_example = generate(rand::thread_rng().gen_range(1..50), charset);
            if !curr_example.starts_with("1") || !curr_example.ends_with("0") {
                if !examples.contains(&curr_example) {
                    examples.push(curr_example);
                }
            }
        }
    }

    examples
}
