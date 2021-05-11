use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        _n: usize,
        k: Usize1,
        mut s: Chars,
    };
    s[k] = s[k].to_ascii_lowercase();
    let ans = s.iter().collect::<String>();
    println!("{}", ans);
}
