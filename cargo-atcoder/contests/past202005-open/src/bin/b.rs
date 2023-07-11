use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
    };
    let mut resolved = vec![vec![false; m]; n];
    let mut count = vec![0; m];
    for _ in 0..q {
        input! { t_i: usize };
        match t_i {
            1 => {
                input! { n_i: Usize1 };
                let mut score = 0;
                for (j, &r_j) in resolved[n_i].iter().enumerate() {
                    if r_j {
                        score += n - count[j];
                    }
                }
                println!("{}", score);
            }
            2 => {
                input! { n_i: Usize1, m_i: Usize1 };
                resolved[n_i][m_i] = true;
                count[m_i] += 1;
            }
            _ => unreachable!(),
        }
    }
}
