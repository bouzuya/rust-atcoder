use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let mut ts = vec![];
    for i in 0..s.len() {
        ts.push(s[i..].iter().chain(s[0..i].iter()).collect::<String>());
    }
    ts.sort();

    println!("{}", ts[0]);
    println!("{}", ts[s.len() - 1]);
}
