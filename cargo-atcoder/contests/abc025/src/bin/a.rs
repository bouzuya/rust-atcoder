use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        n: usize,
    };
    let mut t = vec![];
    for i in 0..s.len() {
        for j in 0..s.len() {
            t.push(vec![s[i], s[j]]);
        }
    }
    t.sort();
    let ans = t[n - 1].iter().collect::<String>();
    println!("{}", ans);
}
