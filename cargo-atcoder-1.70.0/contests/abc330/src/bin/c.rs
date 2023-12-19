use proconio::input;

fn main() {
    input! {
        d: usize,
    };

    let mut ans = 1_usize << 60;
    for x in 1_usize.. {
        let x2 = x.pow(2);
        if x2 > d {
            break;
        }

        let y2q = d - x2;
        let mut ok = 1_usize;
        let mut ng = 1_000_000_000_usize;
        while ng - ok > 1 {
            let mid = (ok + ng) / 2;
            if mid.pow(2) <= y2q {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        for y in [ok.saturating_sub(1), ok, ng, ng + 1].iter().copied() {
            let y2 = y.pow(2);
            let z2 = x2 + y2;
            let a = z2.max(d) - z2.min(d);
            ans = ans.min(a);
        }
    }

    println!("{}", ans);
}
