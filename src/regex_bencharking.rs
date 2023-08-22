use cute::{self, c};
use rand::Rng;
use random_string::generate;
use regex::Regex;
use std::time::Instant;

pub fn regex_benchmarking() {
    let charset = "01";
    let set_len: i32 = 1000000000;
    let string_set: Vec<String> =
        c![generate(rand::thread_rng().gen_range(1..50), charset), for _x in 0..set_len];
    let regex_set: [&str; 4] = [
        r"^(0(1)*)*$",
        r"^((1)*0)*1$",
        r"^0((0)*1)*$",
        r"^1((1)*0)*$",
    ];

    for regexp in regex_set {
        let start: Instant = Instant::now();
        string_set
            .iter()
            .all(|x: &String| Regex::new(regexp).unwrap().is_match(x));
        let elapsed_secs: f32 = start.elapsed().as_secs_f32();
        println!(
            "finished in {} sec, set len = {}",
            elapsed_secs,
            string_set.len()
        );
    }
}
