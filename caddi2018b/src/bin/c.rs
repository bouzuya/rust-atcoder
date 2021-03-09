// 解説 AC
use proconio::input;

fn prime_factorization(n: usize) -> Vec<(usize, usize)> {
    let mut p = vec![];
    if n < 2 {
        return p;
    }
    let mut n = n; // shadowing
    for i in 2.. {
        if i * i > n {
            break;
        }
        let mut c = 0;
        while n % i == 0 {
            c += 1;
            n /= i;
        }
        if c > 0 {
            p.push((i, c));
        }
    }
    if n != 1 {
        p.push((n, 1));
    }
    p
}

fn main() {
    input! {
        n: usize,
        p: usize,
    };
    let ps = prime_factorization(p);
    let mut x = 1;
    for (p_i, q_i) in ps {
        x *= p_i.pow((q_i / n) as u32);
    }
    let ans = x;
    println!("{}", ans);
}
