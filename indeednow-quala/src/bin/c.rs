use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        s: [i64; n],
        q: usize,
        k: [usize; q],
    };
    let mut s = s.into_iter().filter(|&s_i| s_i != 0).collect::<Vec<i64>>();
    s.sort();
    let n = s.len();

    for &k_i in k.iter() {
        let mut ng = -1_i64;
        let mut ok = 1_000_001_i64;
        while ok - ng > 1 {
            let x = ng + (ok - ng) / 2;
            if (n - s.lower_bound(&x)) <= k_i {
                ok = x;
            } else {
                ng = x;
            }
        }
        let ans = ok;
        println!("{}", ans);
    }
}
