use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [usize; n],
    };
    let mut count = 0_usize;
    for (i, d_i) in d.iter().copied().enumerate() {
        let i = i + 1;
        for j in 1..=d_i {
            if format!("{}{}", i, j)
                .chars()
                .collect::<HashSet<char>>()
                .len()
                == 1
            {
                count += 1;
            }
        }
    }
    let ans = count;
    println!("{}", ans);
}
