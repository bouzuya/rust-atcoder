use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
        a: usize,
        b: usize,
        c: usize,
        mut av: [usize; a],
        mut bv: [usize; b],
        mut cv: [usize; c],
    };
    av.sort_by(|a, b| b.cmp(&a));
    bv.sort_by(|a, b| b.cmp(&a));
    cv.sort_by(|a, b| b.cmp(&a));

    let mut ans = 0_usize;
    for i in 0..x {
        ans += av[i];
    }
    for i in 0..y {
        ans += bv[i];
    }
    let n_c = std::cmp::min(cv.len(), x + y);
    for i in 0..n_c {
        ans += cv[i];
    }
    let inf = 1_000_000_000 + 1;
    let mut i_a = x;
    let mut i_b = y;
    let mut i_c = n_c;
    for _ in 0..n_c {
        let p = if i_a >= 1 { av[i_a - 1] } else { inf };
        let q = if i_b >= 1 { bv[i_b - 1] } else { inf };
        let r = if i_c >= 1 { cv[i_c - 1] } else { inf };
        ans -= if p <= q && p <= r {
            i_a -= 1;
            p
        } else if q <= p && q <= r {
            i_b -= 1;
            q
        } else {
            i_c -= 1;
            r
        };
    }
    println!("{}", ans);
}
