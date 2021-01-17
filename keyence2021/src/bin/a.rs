use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        b: [i64; n],
    };
    let mut c = vec![];
    let mut max_a = a[0];
    for i in 0..n {
        let x = if c.is_empty() { 0 } else { c[c.len() - 1] };
        if max_a < a[i] {
            max_a = a[i];
        }
        c.push(max(x, max_a * b[i]));
    }
    for c_i in c {
        println!("{}", c_i);
    }
}
