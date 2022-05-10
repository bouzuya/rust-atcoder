use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let m = 1_000_000;
    let mut primes = vec![];
    let mut cum = vec![1; m + 1];
    cum[0] = 0;
    cum[1] = 0;
    for i in 2.. {
        if i * i > m {
            for j in i..=m {
                if cum[j] == 1 {
                    primes.push(j);
                }
            }
            break;
        }
        if cum[i] == 1 {
            primes.push(i);
            for j in (i + i..=m).step_by(i) {
                cum[j] = 0;
            }
        }
    }
    for i in 0..1_000_000 {
        cum[i + 1] += cum[i];
    }

    let mut count = 0_usize;
    for q in primes.iter().copied() {
        let q3 = q * q * q;
        if q3 > n {
            break;
        }
        count += cum[(n / q3).min(q - 1)];
    }

    let ans = count;
    println!("{}", ans);
}
