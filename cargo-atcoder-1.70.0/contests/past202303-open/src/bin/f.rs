use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [usize; n],
        q: usize,
    };
    let s = s.into_iter().collect::<HashSet<usize>>();
    for _ in 0..q {
        input! {
            m: usize,
            t: [usize; m],
        }
        let mut count = s.len();
        for t_i in t {
            if !s.contains(&t_i) {
                count += 1;
            }
        }
        println!("{}", count);
    }
}
