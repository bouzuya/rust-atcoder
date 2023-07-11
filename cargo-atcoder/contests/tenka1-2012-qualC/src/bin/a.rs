use proconio::input;

fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
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
    p
}

fn main() {
    input! {
        n: usize,
    };
    let ans = sieve_of_eratosthenes(n - 1).len();
    println!("{}", ans);
}
