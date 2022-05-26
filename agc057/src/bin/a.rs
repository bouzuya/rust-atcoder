use proconio::input;

fn main() {
    input! {
        t: usize,
        case: [(usize, usize); t],
    };
    let f = |x: usize| (x * 10).min(10_usize.pow(x.to_string().len() as u32) + x);
    for (l, r) in case {
        let ans = if f(l) > r {
            r - l + 1
        } else {
            let mut ok = r;
            let mut ng = l;
            while ok - ng > 1 {
                let x = ng + (ok - ng) / 2;
                if f(x) > r {
                    ok = x;
                } else {
                    ng = x;
                }
            }
            r - ok + 1
        };
        println!("{}", ans);
    }
}
