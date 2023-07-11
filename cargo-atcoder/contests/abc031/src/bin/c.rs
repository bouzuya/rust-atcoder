use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let inf = 1_000_000_000_i64;
    let mut ans = -inf;
    for i in 0..n {
        let mut max_s = -inf;
        let mut max_j = 0;
        for j in 0..n {
            if i == j {
                continue;
            }
            let l = if i < j { i } else { j };
            let r = if i < j { j } else { i };

            let mut s = 0;
            let mut c = 0;
            for k in l..=r {
                if c % 2 != 0 {
                    s += a[k];
                }
                c += 1;
            }
            if s > max_s {
                max_s = s;
                max_j = j;
            }
        }
        let l = if i < max_j { i } else { max_j };
        let r = if i < max_j { max_j } else { i };

        let mut s = 0;
        let mut c = 0;
        for k in l..=r {
            if c % 2 == 0 {
                s += a[k];
            }
            c += 1;
        }
        ans = std::cmp::max(ans, s);
    }
    println!("{}", ans);
}
