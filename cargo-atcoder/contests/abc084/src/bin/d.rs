use proconio::input;

fn sieve_of_eratosthenes(n: usize) -> Vec<bool> {
    let mut p = vec![];
    let mut b = vec![true; n + 1];
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
        q: usize,
        lr: [(usize, usize); q],
    };
    let max = 100_000;
    let sieve = sieve_of_eratosthenes(max);
    let mut count = vec![0_usize; max + 1];
    for i in 3..=max {
        count[i] = if i % 2 != 0 && sieve[i] && sieve[(i + 1) / 2] {
            1
        } else {
            0
        };
    }
    for i in 1..max {
        count[i + 1] += count[i];
    }
    for (l, r) in lr {
        println!("{}", count[r] - count[l - 1]);
    }
}
