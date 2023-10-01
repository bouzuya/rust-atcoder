use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let n = s.len();
    let mut max = 1_usize;
    for l in 0..n {
        for r in l..n {
            let mut t = s[l..=r].to_vec();
            t.reverse();
            if s[l..=r] == t {
                max = max.max(r - l + 1);
            }
        }
    }
    let ans = max;
    println!("{}", ans);
}
