use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        k: usize,
    };
    let mut ss = vec![false; n];
    for _ in 0..k {
        input! {
            d: usize,
            a: [Usize1; d],
        };
        for ai in a {
            ss[ai] = true;
        }
    }
    let ans = ss.iter().filter(|&s| !s).count();
    println!("{}", ans);
}
