use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
        lrv: [(Usize1, Usize1, i64); q],
    };
    let mut x = 0;
    // a[0]    a[1]
    //     b[0]
    let mut b = vec![0; n];
    let mut p = a[0];
    for (i, &a_i) in a.iter().enumerate().skip(1) {
        b[i] = a_i - p;
        x += b[i].abs();
        p = a_i;
    }
    for (l_i, r_i, v_i) in lrv {
        if l_i > 0 {
            x -= b[l_i].abs();
            b[l_i] += v_i;
            x += b[l_i].abs();
        }
        if r_i < n - 1 {
            x -= b[r_i + 1].abs();
            b[r_i + 1] -= v_i;
            x += b[r_i + 1].abs();
        }
        let ans = x;
        println!("{}", ans);
    }
}
