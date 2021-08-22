use proconio::{input, marker::Chars};
use superslice::Ext;

fn main() {
    input! {
        mut s: Chars,
        k: usize,
    };
    s.sort();
    for _ in 0..k - 1 {
        s.next_permutation();
    }
    println!("{}", s.into_iter().collect::<String>());
}
