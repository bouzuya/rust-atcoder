use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        lrt: [(Usize1, Usize1, usize); q],
    };
    let mut a = vec![0_usize; n];
    for (l, r, t) in lrt {
        for i in l..=r {
            a[i] = t;
        }
    }
    for a_i in a {
        println!("{}", a_i);
    }
}
