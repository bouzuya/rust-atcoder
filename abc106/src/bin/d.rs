use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        lrv: [(usize, usize); m],
        pqv: [(usize, usize); q],
    };
    let mut cvv = vec![vec![0usize; n + 1]; n + 1];
    for (l, r) in lrv {
        cvv[l][r] += 1;
    }
    for l in 1..=n {
        for r in 1..=n {
            cvv[l][r] += cvv[l - 1][r];
        }
    }
    for l in 1..=n {
        for r in 1..=n {
            cvv[l][r] += cvv[l][r - 1];
        }
    }
    for (p, q) in pqv {
        println!(
            "{}",
            cvv[q][q] - cvv[p - 1][q] - cvv[q][p - 1] + cvv[p - 1][p - 1]
        );
    }
}
