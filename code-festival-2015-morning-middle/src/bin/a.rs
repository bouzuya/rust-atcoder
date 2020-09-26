use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        m: usize,
        r: usize,
        mut s: [usize; n - 1]
    };
    s.sort();
    s.reverse();
    let sum = s.iter().take(k).sum::<usize>();
    let s_k = if k == n { 0 } else { s[k - 1] };
    let ans = if sum / k >= r {
        0_i64
    } else {
        if (sum - s_k + m) / k < r {
            -1
        } else {
            let mut ng = 0;
            let mut ok = m;
            while ok - ng > 1 {
                let x = ng + (ok - ng) / 2;
                if (sum - s_k + x) / k >= r {
                    ok = x;
                } else {
                    ng = x;
                }
            }
            ok as i64
        }
    };
    println!("{}", ans);
}
