use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };
    let ans = s
        .iter()
        .zip(t.iter())
        .filter(|&(&s_i, &t_i)| s_i != t_i)
        .count();
    println!("{}", ans);
}
