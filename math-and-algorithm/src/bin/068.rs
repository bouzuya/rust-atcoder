use proconio::input;

fn gcd(n: usize, m: usize) -> usize {
    if m == 0 {
        n
    } else {
        gcd(m, n % m)
    }
}

fn lcm(n: usize, m: usize) -> usize {
    n * m / gcd(n, m)
}

fn main() {
    input! {
        n: usize,
        k: usize,
        v: [usize; k],
    };

    let mut ans = 0_i64;
    for bits in 1..1 << k {
        let mut is_even = true;
        let mut x = 1_usize;
        for (i, v_i) in v.iter().copied().enumerate() {
            if ((bits >> i) & 1) == 1 {
                x = lcm(x, v_i);
                is_even = !is_even;
            }
        }
        ans += if is_even { -1 } else { 1 } * ((n / x) as i64);
    }

    println!("{}", ans);
}
