use std::collections::BTreeSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let mut ok = false;
    for i in 0..26 {
        let c = (b'A' + i) as char;
        if s.contains(&c) {
            ok = true;
            break;
        }
    }
    if !ok {
        println!("No");
        return;
    }

    let mut ok = false;
    for i in 0..26 {
        let c = (b'a' + i) as char;
        if s.contains(&c) {
            ok = true;
            break;
        }
    }
    if !ok {
        println!("No");
        return;
    }

    let set = s.iter().collect::<BTreeSet<_>>();
    if set.len() != s.len() {
        println!("No");
        return;
    }

    println!("Yes");
}
