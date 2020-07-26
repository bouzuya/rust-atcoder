use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        q: usize,
        lr: [(Usize1, usize); q],
    };
    let mut c = vec![0_i64; n + 1];
    for &(l, r) in lr.iter() {
        c[l] += 1;
        c[r] -= 1;
    }
    let mut ans = vec![0_i64; n];
    ans[0] = c[0];
    for i in 1..n {
        ans[i] += ans[i - 1] + c[i];
    }
    for i in 0..n {
        ans[i] %= 2;
        print!("{}", ans[i]);
    }
    println!();
}
