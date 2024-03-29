use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        tx: [(usize, Usize1); q],
    };

    let mut count = vec![0_usize; n];
    for (t, x) in tx {
        match t {
            1 => count[x] += 1,
            2 => count[x] += 2,
            3 => println!("{}", if count[x] >= 2 { "Yes" } else { "No" }),
            _ => unreachable!(),
        }
    }
}
