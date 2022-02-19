use proconio::input;

fn sieve_of_eratosthenes(n: usize) -> Vec<bool> {
    let mut p = vec![];
    let mut b = vec![true; n + 1];
    b[0] = false;
    b[1] = false;
    for i in 2.. {
        if i * i > n {
            for j in i..=n {
                if b[j] {
                    p.push(j);
                }
            }
            break;
        }
        if b[i] {
            p.push(i);
            for j in (i + i..=n).step_by(i) {
                b[j] = false;
            }
        }
    }
    b
}

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    };

    let mut ans = false;
    let ps = sieve_of_eratosthenes(b + d + 1);
    for x in a..=b {
        let mut contains = false;
        for y in c..=d {
            if ps[x + y] {
                contains = true;
            }
        }
        if !contains {
            ans = true;
        }
    }
    println!("{}", if ans { "Takahashi" } else { "Aoki" });
}
