#[allow(dead_code)]
mod regex_bencharking;
mod utils;
use flame as f;
use flamer::flame;
use std::fs::File;
use utils::State;
// use std::env;
use std::collections::{linked_list, LinkedList};
use std::time::{Duration, Instant};

#[derive(Debug)]
struct TestCase {
    positive_set: Vec<String>,
    negative_set: Vec<String>,
}

impl TestCase {
    fn new(positive_set: Vec<String>, negative_set: Vec<String>) -> TestCase {
        TestCase {
            positive_set,
            negative_set,
        }
    }

    fn synth(&self, debug: bool) -> utils::State {
        let start: Instant = Instant::now();
        let state: utils::State = utils::synth(&self.positive_set, &self.negative_set, debug);
        let elapsed: Duration = start.elapsed();
        let elapsed_secs: f32 = elapsed.as_secs_f32();
        println!("{}, {}, {:?}", state.regexp, state.cost, state.parentheses);
        println!("finished in {} seconds.", elapsed_secs);
        state
    }
}

#[flame]
fn main() {
    // env::set_var("RUST_BACKTRACE", "1");
    let neg_set_len: usize = 1000;
    let cases: Vec<TestCase> = vec![
        // start with 0
        TestCase::new(
            vec!["01", "01101", "0001"]
                .iter()
                .map(|&x| x.to_string())
                .collect(),
            // utils::negative_examples("start_with_0", neg_set_len),
            vec!["10", "1", "11010"]
                .iter()
                .map(|&x| x.to_string())
                .collect(),
        ),
        // end with 01
        TestCase::new(
            vec!["101", "001101101", "0110001"]
                .iter()
                .map(|&x| x.to_string())
                .collect(),
            // utils::negative_examples("end_with_01", neg_set_len),
            vec!["100101011", "110000", "00111010"]
                .iter()
                .map(|&x| x.to_string())
                .collect(),
        ),
        // begin with 1 and end with 0
        TestCase::new(
            vec![
                "11101001010010101000",
                "100101001011101011100",
                "10010111010010100010",
            ]
            .iter()
            .map(|&x| x.to_string())
            .collect(),
            // utils::negative_examples("begin_1_end_0", neg_set_len),
            vec![
                "101001010010101000111",
                "00010101010100100010110",
                "00010101010101001011",
                "0011010100110000001111010100",
                "1001010010101011111111001011",
            ]
            .iter()
            .map(|&x| x.to_string())
            .collect(),
        ),
        // containing the substring 0101
        TestCase::new(
            vec!["0101", "00101001", "000101111"]
                .iter()
                .map(|&x| x.to_string())
                .collect(),
            // utils::negative_examples("containing_0101", neg_set_len),
            vec!["10", "1", "11010", "1001100", "00100010", "0110110"]
                .iter()
                .map(|&x| x.to_string())
                .collect(),
        ),
        // // length is at least 3 and the third symbol is 0
        // TestCase::new(
        //     vec!["110", "0100010100", "000111"]
        //         .iter()
        //         .map(|&x| x.to_string())
        //         .collect(),
        //     vec!["10", "101100", "0", "111000"]
        //         .iter()
        //         .map(|&x| x.to_string())
        //         .collect(),
        // ),
        // // each 0 is followed by at least one 1
        // TestCase::new(
        //     vec![
        //         "01",
        //         "1010111011101011101011101",
        //         "01011011101111011111",
        //         "011010111",
        //         "11010110101111",
        //         "01111",
        //         "1101",
        //     ]
        //     .iter()
        //     .map(|&x| x.to_string())
        //     .collect(),
        //     vec![
        //         "0000",
        //         "01110001011",
        //         "011010000",
        //         "0110001",
        //         "0001011010",
        //         "00101100100",
        //     ]
        //     .iter()
        //     .map(|&x| x.to_string())
        //     .collect(),
        // ),
    ];

    for c in &cases {
        c.synth(false);
    }

    // f::dump_html(File::create("flamegraph2.html").unwrap()).unwrap();
    // f::dump_json(&mut File::create("flamegraph.json").unwrap()).unwrap();
}

#[cfg(test)]
mod test {
    use std::collections::LinkedList;

    use super::*;

    #[test]
    fn start_with_0() {
        let ps: Vec<String> = vec!["01".to_string(), "01101".to_string(), "0001".to_string()];
        let ns: Vec<String> = vec!["10".to_string(), "1".to_string(), "11010".to_string()];
        let state: utils::State = utils::synth(&ps, &ns, false);
        assert_eq!(
            state,
            utils::State::new(5, "^(0(1)*)*$".to_string(), Vec::from([(1, 7), (3, 5)]))
        )
    }

    #[test]
    fn update_test() {
        let s = State::new(0, "^(abc)\x00(def)$".to_string(), vec![(1, 5), (10, 14)]);
        let mut ext_parentheses = s.parentheses.clone();
        utils::update_parentheses(&mut ext_parentheses, 7, 3, true);
        assert_eq!(ext_parentheses, vec![(1, 5), (7, 11)]);
    }

    #[test]
    fn is_inside_or_works() {
        let s: State = State::new(
            4,
            "^(((\x00|\x00|\x00))*)*$".to_string(),
            [(1, 21), (2, 19), (3, 18)].to_vec(),
        );
        assert!(utils::is_inside_or(&s, 14) == true);
    }
    // #[test]
    // fn vec_bench() {
    //     // Create a vector of usize with elements from 0 to 999,999
    //     let v: Vec<usize> = (0..=9999999).collect();

    //     // Iterate through the vector and assert that each element is equal to its index
    //     for (i, &element) in v.iter().enumerate() {
    //         assert!(element == i);
    //     }
    // }

    // #[test]
    // fn ll_bench() {
    //     let v: Vec<usize> = (0..=9999999).collect();
    //     let mut ll: LinkedList<usize> = LinkedList::new();
    //     ll.extend(v.clone());

    //     for (i, &value) in ll.iter().enumerate() {
    //         assert!(value == v[i]);
    //     }
    // }
}
