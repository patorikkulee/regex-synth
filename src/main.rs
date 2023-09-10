#[allow(dead_code)]
mod regex_bencharking;
mod utils;
use flame as f;
use flamer::flame;
use std::fs::File;
// use std::env;
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

    fn synth(&self, debug: bool) -> (String, usize) {
        let start: Instant = Instant::now();
        let state: (String, usize) = utils::synth(&self.positive_set, &self.negative_set, debug);
        let elapsed: Duration = start.elapsed();
        let elapsed_secs: f32 = elapsed.as_secs_f32();
        println!("{:?}", state);
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

    for c in &cases[..1] {
        c.synth(false);
    }

    // fn find_char_occurrences(input_str: &str) -> Vec<(usize, usize)> {
    //     let left: Vec<usize> = input_str
    //         .char_indices()
    //         .filter(|&(_, c)| c == '(')
    //         .map(|(index, _)| index)
    //         .collect();

    //     let right: Vec<usize> = input_str
    //         .char_indices()
    //         .filter(|&(_, c)| c == ')')
    //         .map(|(index, _)| index)
    //         .rev()
    //         .collect();

    //     let indices: Vec<(usize, usize)> = left
    //         .iter()
    //         .zip(right.iter())
    //         .map(|(&l, &r)| (l, r))
    //         .collect();
    //     indices
    // }
    // let tmp: Vec<(usize, usize)> = find_char_occurrences("sdf(sdga(f|h)gsd)asdf");
    // println!("{:?}", tmp);
    // let ts: String = "sdf(sdga(f|h)gsd)asdf".to_string();
    // println!("{:?}", utils::find_parentheses(&ts, false));

    // let mut queue: utils::Queue = utils::Queue::new(
    //     vec![
    //         vec!["a".to_string()],
    //         vec!["b".to_string()],
    //         vec!["c".to_string()],
    //     ],
    //     0,
    //     0,
    // );
    // queue.push("hi".to_string(), 2);
    // println!("{:?}", queue.pop());
    // println!("{:?}", queue.pop());
    // println!("{:?}", queue.pop());
    // println!("{:?}", queue.pop());
    f::dump_html(File::create("flamegraph.html").unwrap()).unwrap();
    f::dump_json(&mut File::create("flamegraph.json").unwrap()).unwrap();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn start_with_0() {
        let ps: Vec<String> = vec!["01".to_string(), "01101".to_string(), "0001".to_string()];
        let ns: Vec<String> = vec!["10".to_string(), "1".to_string(), "11010".to_string()];
        let state: (String, usize) = utils::synth(&ps, &ns, false);
        assert_eq!(state, ("^(0(1)*)*$".to_string(), 5))
    }
}
