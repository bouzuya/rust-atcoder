use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        w: [String; n],
    };
    let mut set = BTreeSet::new();
    let mut p = w[0].chars().last().unwrap();
    set.insert(&w[0]);
    for (i, w_i) in w.iter().enumerate().skip(1) {
        if p == w_i.chars().nth(0).unwrap() && set.insert(w_i) {
            p = w_i.chars().last().unwrap();
            continue;
        }
        println!("{}", if i % 2 == 0 { "LOSE" } else { "WIN" });
        return;
    }
    println!("DRAW");
}
