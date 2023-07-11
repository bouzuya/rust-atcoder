use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        h: [u64; n],
        ab: [(Usize1, Usize1); m],
    };

    let mut ls = vec![n; n];
    for &(a, b) in ab.iter() {
        if h[a] == h[b] {
            ls[b] = a;
            ls[a] = b;
        } else if h[a] > h[b] {
            ls[b] = a;
        } else {
            ls[a] = b;
        }
    }
    let ans = ls.iter().filter(|&&l| l == n).count();
    println!("{}", ans);
}
