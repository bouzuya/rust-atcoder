use proconio::input;
use superslice::*;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; m],
    };
    let mut c = vec![-1; n];
    for &a_i in a.iter() {
        let i = c.upper_bound_by_key(&-a_i, |&c_i| -c_i);
        if i == c.len() {
            println!("-1");
        } else {
            println!("{}", i + 1);
            c[i] = a_i;
        }
    }
}
