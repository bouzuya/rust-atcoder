use proconio::input;

fn gcd(n: usize, m: usize) -> usize {
    if m == 0 {
        n
    } else {
        gcd(m, n % m)
    }
}

fn lcm(n: usize, m: usize) -> usize {
    n / gcd(n, m) * m
}

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    };

    let mut l = 1_usize;
    for &a_i in a.iter() {
        let b_i = a_i / 2;
        l = lcm(l, b_i);
    }

    for &a_i in a.iter() {
        let b_i = a_i / 2;
        if (l / b_i) % 2 == 0 {
            println!("0");
            return;
        }
    }

    let ans = (m / l + 1) / 2;
    println!("{}", ans);
}
