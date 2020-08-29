// TLE
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
        a: [u64; n],
    };

    let mut is_pc = true;
    let mut x = a[0];
    for i in 0..n - 1 {
        x = gcd(x, a[i + 1]);
        if !is_pc {
            continue;
        }
        let a_i = a[i];
        for j in i + 1..n {
            let a_j = a[j];
            if gcd(a_i, a_j) != 1 {
                is_pc = false;
                break;
            }
        }
    }
    let is_sc = x == 1;
    let ans = if is_pc {
        "pairwise coprime"
    } else if is_sc {
        "setwise coprime"
    } else {
        "not coprime"
    };
    println!("{}", ans);
}
