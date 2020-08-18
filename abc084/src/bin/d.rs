use proconio::input;

fn main() {
    input! {
        q: usize,
        lr: [(i64, i64); q],
    };

    let n = *lr.iter().map(|(_, r)| r).max().unwrap() as usize;

    let mut sieve = vec![true; n + 1];
    sieve[0] = false;
    sieve[1] = false;
    for i in 2.. {
        if i * i > n {
            break;
        }
        if sieve[i] {
            for j in (i + i..=n).step_by(i) {
                sieve[j] = false;
            }
        }
    }

    let mut c = vec![0; n + 2];
    for i in 1..=n {
        c[i] = c[i - 1]
            + if i % 2 == 0 {
                0
            } else {
                if sieve[i] && sieve[(i + 1) / 2] {
                    1
                } else {
                    0
                }
            };
    }

    for &(l_i, r_i) in lr.iter() {
        println!("{}", c[r_i as usize] - c[(l_i - 1) as usize]);
    }
}
