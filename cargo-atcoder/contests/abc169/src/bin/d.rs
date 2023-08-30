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
    };
    let ps = prime_factorization(n);
    let mut ans = 0_usize;
    for (_, q) in ps {
        let mut count = 0_usize;
        let mut sum = 0_usize;
        for i in 1..=q {
            sum += i;
            if sum > q {
                break;
            }
            count += 1;
        }
        ans += count;
    }
    println!("{}", ans);
}
