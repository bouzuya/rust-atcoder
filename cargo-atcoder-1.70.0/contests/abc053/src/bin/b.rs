use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let l = s.iter().copied().position(|s_i| s_i == 'A').unwrap();
    let r = s.iter().copied().rposition(|s_j| s_j == 'Z').unwrap();
    let ans = r - l + 1;
    println!("{}", ans);
}
