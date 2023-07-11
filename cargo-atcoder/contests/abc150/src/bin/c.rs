use proconio::{input, marker::Usize1};
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        p: [Usize1; n],
        q: [Usize1; n],
    };
    let f = |v: &Vec<usize>| -> usize {
        let mut x = (0..n).collect::<Vec<usize>>();
        for i in 0.. {
            if &x == v {
                return i;
            }
            if !x.next_permutation() {
                break;
            }
        }
        unreachable!()
    };
    let a = f(&p) as i64;
    let b = f(&q) as i64;
    let ans = (a - b).abs();
    println!("{}", ans);
}
