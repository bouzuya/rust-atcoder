use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
        b: [usize; n],
        c: [usize; n - 1],
    };
    let mut prev = n;
    let mut sum = 0_usize;
    for a_i in a {
        sum += b[a_i];
        if prev + 1 == a_i {
            sum += c[prev];
        }
        prev = a_i;
    }
    let ans = sum;
    println!("{}", ans);
}
