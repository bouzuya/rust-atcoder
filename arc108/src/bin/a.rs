use proconio::input;

fn divisors(n: i64) -> Vec<i64> {
    let mut d = vec![];
    for i in 1.. {
        if i * i > n {
            break;
        }
        if n % i == 0 {
            d.push(i);
            if i != n / i {
                d.push(n / i);
            }
        }
    }
    d.sort();
    d
}

fn main() {
    input! {
        s: i64,
        p: i64,
    };
    // N + M = S
    // N * M = P
    let ds = divisors(p);
    for &n in ds.iter() {
        for &m in ds.iter() {
            if m + n == s && m * n == p {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
