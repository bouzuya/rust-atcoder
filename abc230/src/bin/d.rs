use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        mut lr: [(usize, usize); n],
    };
    lr.sort_by_key(|&(a, b)| (b, a));
    let mut ok = 1_000_000_000;
    let mut ng = 0;
    while ok - ng > 1 {
        let x = (ok + ng) / 2;
        let mut count = 1_usize;
        let mut p = lr[0].1;
        for (l, r) in lr.iter().skip(1).copied() {
            if p + d - 1 < l {
                count += 1;
                p = r;
            }
        }
        if count <= x {
            ok = x;
        } else {
            ng = x;
        }
    }
    let ans = ok;
    println!("{}", ans);
}
