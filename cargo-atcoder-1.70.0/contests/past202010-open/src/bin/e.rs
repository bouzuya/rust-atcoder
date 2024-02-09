use proconio::{input, marker::Chars};
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        s: Chars,
    };
    let mut is = (0..n).collect::<Vec<usize>>();
    loop {
        let t = is.iter().copied().map(|i| s[i]).collect::<Vec<char>>();
        if s != t && t.iter().rev().copied().collect::<Vec<char>>() != s {
            println!("{}", t.iter().collect::<String>());
            return;
        }

        if !is.next_permutation() {
            break;
        }
    }

    println!("None");
}
