use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
        mut t: Chars,
    };
    if s[0] > s[1] {
        s.swap(0, 1);
    }
    if t[0] > t[1] {
        t.swap(0, 1);
    }

    let mut set1 = HashSet::new();
    set1.insert("AB".chars().collect::<Vec<char>>());
    set1.insert("AE".chars().collect::<Vec<char>>());
    set1.insert("BC".chars().collect::<Vec<char>>());
    set1.insert("CD".chars().collect::<Vec<char>>());
    set1.insert("DE".chars().collect::<Vec<char>>());

    let mut set2 = HashSet::new();
    set2.insert("AC".chars().collect::<Vec<char>>());
    set2.insert("AD".chars().collect::<Vec<char>>());
    set2.insert("BD".chars().collect::<Vec<char>>());
    set2.insert("BE".chars().collect::<Vec<char>>());
    set2.insert("CE".chars().collect::<Vec<char>>());

    let ans = (set1.contains(&s) && set1.contains(&t)) || (set2.contains(&s) && set2.contains(&t));
    println!("{}", if ans { "Yes" } else { "No" });
}
