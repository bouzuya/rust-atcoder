use proconio::{input, marker::Chars};
use std::collections::BTreeSet;

fn main() {
    input! {
        s: Chars,
    };
    let set = s.into_iter().collect::<BTreeSet<_>>();
    for i in 0..26 {
        let c = (b'a' + i) as char;
        if !set.contains(&c) {
            println!("{}", c);
            return;
        }
    }
    println!("None");
}
