use proconio::input;

fn main() {
    input! {
        n: usize,
        q: [usize; n],
        a: [usize; n],
        b: [usize; n],
    };
    let mut ok = 0_usize;
    let mut ng = 10_000_001_usize;
    while ng - ok > 1 {
        let mid = (ok + ng) / 2;

        let mut ok2 = false;
        for x in 0..=mid {
            let y = mid - x;
            if q.iter()
                .copied()
                .zip(a.iter())
                .zip(b.iter())
                .all(|((q, a), b)| q >= a * x + b * y)
            {
                ok2 = true;
                break;
            }
        }
        if ok2 {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    let ans = ok;
    println!("{}", ans);
}
