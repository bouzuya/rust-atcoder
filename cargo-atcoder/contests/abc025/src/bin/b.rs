use proconio::{input, marker::Chars};
use std::cmp::{max, min};

fn main() {
    input! {
        n: usize,
        a: i64,
        b: i64,
        sd: [(Chars, i64); n],
    };
    let mut c = 0_i64;
    for (s_i, d_i) in sd {
        let x = max(min(d_i, b), a);
        let d = match s_i[0] {
            'E' => x,
            'W' => -x,
            _ => unreachable!(),
        };
        c += d;
    }
    let ans = c;
    if ans == 0 {
        println!("{}", 0);
    } else {
        println!("{} {}", if ans > 0 { "East" } else { "West" }, ans.abs());
    }
}
