use std::collections::BTreeSet;

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    };
    let mut set1 = BTreeSet::new();
    let mut set = BTreeSet::new();
    for s_i in s.iter() {
        if set1.insert(s_i) {
            let x = if s_i[0] == '!' {
                s_i[1..].iter().collect::<String>()
            } else {
                s_i[0..].iter().collect::<String>()
            };
            if !set.insert(x.clone()) {
                println!("{}", x);
                return;
            }
        }
    }
    println!("satisfiable");
}
