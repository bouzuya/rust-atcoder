use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let mut t = vec![];
    for s_i in s {
        t.push(s_i);
        t.push(' ');
    }
    t.pop();
    let ans = t.into_iter().collect::<String>();
    println!("{}", ans);
}
