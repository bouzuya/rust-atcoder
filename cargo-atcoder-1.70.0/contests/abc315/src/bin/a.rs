use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    };
    let mut t = String::new();
    for c in s {
        if "aeiou".chars().any(|x| x == c) {
            continue;
        }
        t.push(c);
    }
    let ans = t;
    println!("{}", ans);
}
