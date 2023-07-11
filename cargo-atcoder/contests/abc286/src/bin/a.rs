use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        p: Usize1,
        q: Usize1,
        r: Usize1,
        s: Usize1,
        a: [usize; n],
    };
    let mut b = a.clone();
    for i in p..=q {
        b[r + i - p] = a[i];
        b[i] = a[r + i - p];
    }
    for b_i in b {
        println!("{}", b_i);
    }
}
