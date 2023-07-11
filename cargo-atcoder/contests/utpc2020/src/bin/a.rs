use proconio::input;

fn main() {
    input! {
        n: usize,
        l: i64,
        mut xa: [(i64, i64); n],
    };
    xa.push((l, 0));

    let mut ok = 1_000_000_000_000_000_i64;
    let mut ng = -1;
    while ok - ng > 1 {
        let t = (ok + ng) / 2;
        let mut is_ok = true;

        let mut v = t;
        let mut p = 0;
        for (x, a) in xa.iter().copied() {
            v += x - p;
            v = v.min(t);
            v -= a;
            if v < 0 {
                is_ok = false;
                break;
            }
            p = x;
        }

        if is_ok {
            ok = t;
        } else {
            ng = t;
        }
    }
    let ans = ok;
    println!("{}", ans);
}
