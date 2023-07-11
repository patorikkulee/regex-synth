mod utils;
use std::time::{Duration, Instant};

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

    fn synth(&self, debug: bool) -> (String, i32) {
        let start: Instant = Instant::now();
        let state: (String, i32) =
            utils::synth(self.positive_set.clone(), self.negative_set.clone(), debug);
        let elapsed: Duration = start.elapsed();
        let elapsed_secs: f32 = elapsed.as_secs_f32();
        println!("{:?}", state);
        println!("took {} seconds.", elapsed_secs);
        state
    }
}

fn main() {
    let cases: Vec<TestCase> = vec![TestCase::new(
        vec!["01", "01101", "0001"]
            .iter()
            .map(|&x| x.to_string())
            .collect(),
        vec!["10", "1", "11010"]
            .iter()
            .map(|&x| x.to_string())
            .collect(),
    )];

    for c in cases {
        c.synth(false);
    }
}
