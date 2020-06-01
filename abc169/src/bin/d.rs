use proconio::input;

fn prime_factorization(n: usize) -> std::collections::BTreeMap<usize, usize> {
    let mut p = std::collections::BTreeMap::new();
    if n < 2 {
        return p;
    }
    let mut n = n; // shadowing
    for i in 2.. {
        if i * i > n {
            break;
        }
        while n % i == 0 {
            *p.entry(i).or_insert(0) += 1;
            n /= i;
        }
    }
    if n != 1 {
        *p.entry(n).or_insert(0) += 1;
    }
    p
}

fn main() {
    input! {
        n: u64,
    };
    let pq = prime_factorization(n as usize);
    let mut count = 0;
    for (&_, &q_i) in pq.iter() {
        let mut c = 0;
        for i in 1.. {
            c += i;
            if c > q_i {
                break;
            }
            count += 1;
        }
    }
    let ans = count;
    println!("{}", ans);
}
