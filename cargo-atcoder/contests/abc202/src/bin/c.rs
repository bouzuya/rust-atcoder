use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
        c: [Usize1; n],
    };

    let mut ca = vec![0; 100_000 + 1];
    for a_i in a {
        ca[a_i] += 1;
    }

    let mut cb = vec![0; 100_000 + 1];
    for c_i in c {
        cb[b[c_i]] += 1;
    }

    let mut count = 0_usize;
    for i in 0..=100_000 {
        count += ca[i] * cb[i];
    }

    let ans = count;
    println!("{}", ans);
}
