use proconio::input;

fn gcd(n: u64, m: u64) -> u64 {
    if n < m {
        gcd(m, n)
    } else if m == 0 {
        n
    } else {
        gcd(m, n % m)
    }
}

fn main() {
    input! {
        n: usize,
        x_start: u64,
        mut x: [u64; n],
    };
    if n == 1 {
        println!(
            "{}",
            if x[0] > x_start {
                x[0] - x_start
            } else {
                x_start - x[0]
            }
        );
        return;
    }
    x.sort();
    let mut d = x[1] - x[0];
    let mut p = x[0];
    for &x_i in x.iter() {
        d = gcd(d, x_i - p);
        p = x_i;
    }
    let ans = if x_start < x[0] {
        gcd(d, x[0] - x_start)
    } else if *x.last().unwrap() < x_start {
        gcd(d, x_start - x.last().unwrap())
    } else {
        let mut res = None;
        for (x_i0, x_i1) in x.windows(2).map(|w| match w {
            &[x_i0, x_i1] => (x_i0, x_i1),
            _ => unreachable!(),
        }) {
            if (x_i0..=x_i1).contains(&x_start) {
                res = Some(std::cmp::max(
                    gcd(d, x_i1 - x_start),
                    gcd(d, x_start - x_i0),
                ));
            }
        }
        res.unwrap()
    };
    println!("{}", ans);
}
