use std::collections::BTreeSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };
    let n = s.len();
    let m = t.len();
    if n < m {
        println!("UNRESTORABLE");
        return;
    }
    let mut set = BTreeSet::new();
    for i in 0..n - m + 1 {
        let mut ok = true;
        for j in 0..m {
            if s[i + j] != '?' && s[i + j] != t[j] {
                ok = false;
                break;
            }
        }
        if !ok {
            continue;
        }

        let mut x = vec![];
        for c in s[0..i].iter().copied() {
            x.push(if c == '?' { 'a' } else { c });
        }
        for c in t.iter().copied() {
            x.push(c);
        }
        for c in s[i + m..].iter().copied() {
            x.push(if c == '?' { 'a' } else { c });
        }
        set.insert(x);
    }

    let ans = set
        .first()
        .map(|s| s.iter().copied().collect::<String>())
        .unwrap_or_else(|| "UNRESTORABLE".to_string());
    println!("{ans}");
}
