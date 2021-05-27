use proconio::input;
use std::cmp;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let mut ans = 0;
    for l in 0..n {
        let mut max = a[l];
        for r in l..n {
            max = cmp::min(max, a[r]);
            ans = cmp::max(ans, max * (r - l + 1));
        }
    }
    println!("{}", ans);
}
