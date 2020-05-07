use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    };
    let zc = s.iter().filter(|&&c| c == '0').count();
    let oc = s.len() - zc;
    let ans = std::cmp::min(zc, oc) * 2;
    println!("{}", ans);
}
