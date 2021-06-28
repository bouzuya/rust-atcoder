use std::cmp;

use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        t: usize,
        x: usize,
        ab: [(usize, usize); n],
    };
    let mut cur = (0, 0);
    for (a_i, b_i) in ab {
        if b_i >= l && a_i > t {
            println!("forever");
            return;
        }
        if b_i < l {
            cur = (cur.0 + a_i, 0);
            continue;
        }
        assert!(b_i >= l);
        match (cur.1 + a_i).cmp(&t) {
            std::cmp::Ordering::Less => {
                cur = (cur.0 + a_i, cur.1 + a_i);
            }
            std::cmp::Ordering::Equal => {
                cur = (cur.0 + a_i, cur.1 + a_i);
                cur = (cur.0 + x, 0);
            }
            std::cmp::Ordering::Greater => {
                cur = (cur.0 + a_i - (cur.1 + a_i - t), cmp::min(cur.1 + a_i, t));
                cur = (cur.0 + x, 0);
                cur = (cur.0 + a_i, cur.1 + a_i);
            }
        }
        if cur.1 == t {
            cur = (cur.0 + x, 0);
        }
    }
    let ans = cur.0;
    println!("{}", ans);
}
