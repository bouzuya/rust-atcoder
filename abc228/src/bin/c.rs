use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: Usize1,
        p: [[usize; 3]; n],
    };
    let mut s = vec![0; n];
    for i in 0..n {
        for j in 0..3 {
            s[i] += p[i][j];
        }
    }
    let mut t = s.clone();
    t.sort();
    t.reverse();
    let t = t[k];
    for s_i in s {
        let ans = s_i + 300 >= t;
        println!("{}", if ans { "Yes" } else { "No" });
    }
}
