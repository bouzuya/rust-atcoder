use proconio::input;
use std::cmp;

fn main() {
    input! {
        n: usize,
        m: usize,
        t: usize,
        ab: [(usize, usize); m],
    };
    let mut c_n = n;
    let mut c_t = 0;
    for (a_i, b_i) in ab {
        let d = a_i - c_t;
        if d >= c_n {
            println!("No");
            return;
        }
        c_n -= d;
        c_n = cmp::min(n, c_n + b_i - a_i);
        c_t = b_i;
    }
    if t - c_t >= c_n {
        println!("No");
        return;
    }
    println!("Yes");
}
