use proconio::{input, marker::Usize1};

fn main() {
    input! {
        capital_n: usize,
        capital_m: usize,
        q: usize,
    };
    let mut count = vec![0_usize; capital_m];
    let mut resolved = vec![vec![false; capital_m]; capital_n];
    for _ in 0..q {
        input! {
            t: usize,
        }

        match t {
            1 => {
                input! {
                    n: Usize1,
                }
                let ans = resolved[n]
                    .iter()
                    .enumerate()
                    .map(|(i, r)| if *r { capital_n - count[i] } else { 0 })
                    .sum::<usize>();
                println!("{}", ans);
            }
            2 => {
                input! {
                    n: Usize1,
                    m: Usize1,
                }
                resolved[n][m] = true;
                count[m] += 1;
            }
            _ => unreachable!(),
        }
    }
}
