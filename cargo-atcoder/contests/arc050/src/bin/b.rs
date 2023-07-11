// 解説 AC
use proconio::input;

fn is_ok(r: usize, b: usize, x: usize, y: usize, k: usize) -> bool {
    if r < k || b < k {
        return false;
    }
    ((r - k) / (x - 1)) + ((b - k) / (y - 1)) >= k
}

fn main() {
    input! {
        r: usize,
        b: usize,
        x: usize,
        y: usize,
    };

    let mut ok = 0;
    let mut ng = 1_000_000_000_000_000_001;
    while ng - ok > 1 {
        let k = ok + (ng - ok) / 2;
        if is_ok(r, b, x, y, k) {
            ok = k;
        } else {
            ng = k;
        }
    }
    let ans = ok;
    println!("{}", ans);
}
