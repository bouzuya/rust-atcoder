use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        t: u64,
        ct: [(u64, u64); n],
    };
    let mut ans = None;
    for (c_i, t_i) in ct {
        if t_i > t {
            continue;
        }
        ans = match ans {
            None => Some(c_i),
            Some(min_c) => Some(min(min_c, c_i)),
        };
    }
    println!(
        "{}",
        match ans {
            None => "TLE".to_string(),
            Some(c) => format!("{}", c),
        }
    );
}
