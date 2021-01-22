use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        k: [i64; n - 1],
    };
    let mut l = vec![0; n];
    l[0] = k[0];
    l[n - 1] = k[n - 2];
    for i in 0..n - 2 {
        l[i + 1] = min(k[i], k[i + 1]);
    }
    for (i, l_i) in l.iter().enumerate() {
        print!("{}{}", l_i, if i == n - 1 { "\n" } else { " " });
    }
}
