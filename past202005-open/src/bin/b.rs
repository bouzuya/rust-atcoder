use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
    };
    let mut a = vec![vec![false; m]; n];
    let mut c = vec![0; m];
    for _ in 0..q {
        input! {
            q_t: usize,
            q_n: Usize1,
        };
        match q_t {
            1 => println!(
                "{}",
                a[q_n]
                    .iter()
                    .enumerate()
                    .filter(|&(_, &ok)| ok)
                    .map(|(i, _)| n - c[i])
                    .sum::<usize>()
            ),
            2 => {
                input! { q_m: Usize1 };
                a[q_n][q_m] = true;
                c[q_m] += 1;
            }
            _ => unreachable!(),
        }
    }
}
