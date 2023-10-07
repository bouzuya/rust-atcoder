use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let mut t = vec![];
    let mut p = ' ';
    for s_i in s {
        if s_i == p {
            continue;
        }
        t.push(s_i);
        p = s_i;
    }

    let ans = t.len() - 1;
    println!("{}", ans);
}
