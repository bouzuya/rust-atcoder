use proconio::input;

fn main() {
    input! {
        mut n: usize,
        mut m: usize,
    };
    let mut ng = 1_000_000_000_000_usize;
    let mut ok = 0_usize;
    while ng - ok > 1 {
        let x = ok + (ng - ok) / 2;
        let y = x.saturating_sub(n);
        if (x <= n && 2 * x <= m) || (2 * (x + y) <= m) {
            ok = x;
        } else {
            ng = x;
        }
    }
    let ans = ok;
    println!("{}", ans);
}
