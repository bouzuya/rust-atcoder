use proconio::input;

fn main() {
    input! {
        r: usize,
        b: usize,
        x: usize,
        y: usize,
    }

    let mut ok = 0_usize;
    let mut ng = r + b;
    while ng - ok > 1 {
        let mid = ok + (ng - ok) / 2;
        if (mid <= r) && (mid <= b) && (((r - mid) / (x - 1)) + ((b - mid) / (y - 1)) >= mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    let ans = ok;
    println!("{}", ans);
}
